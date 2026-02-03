use std::{
    collections::BTreeMap,
    io::{Read, Seek, Write},
};

use anyhow::Context;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use fjall::{Database, Keyspace, KeyspaceCreateOptions, Readable};
use serde::{Deserialize, Serialize};
use zip::{ZipArchive, write::SimpleFileOptions};

// This file supports serializing a bunch of Fjall keyspaces to a file
// to be sent as a Raft snapshot. In the future, when
// https://github.com/fjall-rs/fjall/issues/52 is done, we should use that to just
// take a backup and then tar it up, but for now we need to actually
// separately serialize the whole thing.
//
// The format is as follows:
//
// - The magic data "COYOTE01"
// - A zip snapshot
//
// Inside the zip snapshot is a file called "manifest.json" containing
// the manifest structure below, and then a series of chunk files containing
// sorted rows from each keyspace.
//
// Each chunk is serialized as
//
// [ key length ] [ data length ] [ key ] [ data ]
//
// Where `key length` is a 16-bit big-endian integer, `data length` is a
// 32-bit big-endian integer, and key and data are untransformed bytes.

#[derive(Debug, Serialize, Deserialize)]
struct Chunk {
    num_records: u32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Manifest {
    keyspaces: BTreeMap<String, Vec<Chunk>>,
}

struct KeyspaceSerializer<'a, B: Write + Seek> {
    file: &'a mut zip::ZipWriter<B>,
    keyspace_name: &'a str,
    current_buffer: Vec<u8>,
    current_index: u32,
    current_count: u32,
    chunk_metadata: Vec<Chunk>,
    total_written: usize,
}

impl<'a, B: Write + Seek> KeyspaceSerializer<'a, B> {
    const MAX_BYTES_PER_CHUNK: usize = 10_000_000;

    fn new(file: &'a mut zip::ZipWriter<B>, keyspace_name: &'a str) -> anyhow::Result<Self> {
        let options = SimpleFileOptions::default().unix_permissions(0o755);
        file.add_directory(keyspace_name, options)?;
        Ok(Self {
            file,
            keyspace_name,
            current_buffer: vec![],
            chunk_metadata: vec![],
            current_index: 0,
            current_count: 0,
            total_written: 0,
        })
    }

    fn flush_current(&mut self) -> anyhow::Result<()> {
        if self.current_buffer.is_empty() {
            return Ok(());
        }
        let name = format!("{}/part{}", self.keyspace_name, self.current_index);
        self.current_index += 1;
        self.chunk_metadata.push(Chunk {
            num_records: self.current_count,
            name: name.clone(),
        });
        let options = SimpleFileOptions::default()
            .large_file(true)
            .unix_permissions(0o644);
        self.file.start_file(name, options)?;
        self.file.write_all(&self.current_buffer)?;
        self.current_buffer.clear();
        self.current_count = 0;
        Ok(())
    }

    fn serialize_keyspace<R: Readable>(
        mut self,
        snapshot: &R,
        keyspace: &Keyspace,
    ) -> anyhow::Result<Vec<Chunk>> {
        tracing::trace!(keyspace_name = self.keyspace_name, "serializing keyspace");
        for guard in snapshot.iter(keyspace) {
            let (k, v) = guard.into_inner()?;
            if self.current_buffer.len() > Self::MAX_BYTES_PER_CHUNK {
                self.flush_current()?;
            }
            self.current_count += 1;
            self.current_buffer.write_u16::<BigEndian>(k.len() as u16)?;
            self.current_buffer.write_u32::<BigEndian>(v.len() as u32)?;
            self.current_buffer.write_all(&k)?;
            self.current_buffer.write_all(&v)?;
            self.total_written += 1;
        }
        self.flush_current()?;
        tracing::trace!(
            keyspace_name = self.keyspace_name,
            total_written = self.total_written,
            "finished serializing keyspace"
        );
        Ok(self.chunk_metadata)
    }
}

fn deserialize_keyspace<R: Read + Seek>(
    z: &mut ZipArchive<R>,
    keyspace: &Keyspace,
    chunks: Vec<Chunk>,
) -> anyhow::Result<()> {
    keyspace.clear()?;
    let mut key_buf = vec![];
    let mut value_buf = vec![];

    for chunk in chunks {
        tracing::trace!(?chunk, "deserializing chunk");
        let mut entry = z.by_name(&chunk.name)?;
        let mut i = keyspace.start_ingestion()?;
        for _ in 0..chunk.num_records {
            let key_len = entry.read_u16::<BigEndian>()?;
            let value_len = entry.read_u32::<BigEndian>()?;
            key_buf.resize(key_len as usize, 0u8);
            entry.read_exact(&mut key_buf)?;
            if value_len > 0 {
                value_buf.resize(value_len as usize, 0u8);
                entry.read_exact(&mut value_buf)?;
            } else {
                value_buf.clear();
            }
            i.write(&key_buf, &value_buf)?;
        }
        i.finish()?;
    }
    Ok(())
}

#[tracing::instrument(skip(db, snapshot, file))]
pub(crate) fn serialize_to_file<F: Write + Seek>(
    db: Database,
    snapshot: fjall::Snapshot,
    keyspaces: Vec<String>,
    file: &mut F,
) -> anyhow::Result<()> {
    file.write_all(b"COYOTE01")?;

    let mut zip = zip::write::ZipWriter::new(file);

    let mut manifest = Manifest::default();

    for keyspace_name in keyspaces {
        tracing::debug!(keyspace=%keyspace_name, "serializing a keyspce");
        let keyspace = db.keyspace(&keyspace_name, KeyspaceCreateOptions::default)?;
        let serializer = KeyspaceSerializer::new(&mut zip, &keyspace_name)?;
        let chunks = serializer.serialize_keyspace(&snapshot, &keyspace)?;
        manifest.keyspaces.insert(keyspace_name, chunks);
    }

    let serialized_manifest = serde_json::to_vec(&manifest)?;

    let options = SimpleFileOptions::default()
        .large_file(false)
        .unix_permissions(0o644);
    zip.start_file("manifest.json", options)?;
    zip.write_all(&serialized_manifest)?;

    Ok(())
}

#[tracing::instrument(skip_all)]
pub(crate) fn load_from_file<F: Read + Seek>(db: &Database, f: &mut F) -> anyhow::Result<()> {
    let mut magic = [0u8; 8];
    f.read_exact(&mut magic)?;
    if &magic != b"COYOTE01" {
        panic!("unhandled snapshot format {magic:?}");
    }

    let mut z = zip::read::ZipArchive::new(f)?;

    let Ok(manifest) = z.by_name("manifest.json") else {
        anyhow::bail!("no manifest.json in archive");
    };

    let manifest: Manifest = serde_json::from_reader(manifest).context("parsing manifest")?;

    for (keyspace_name, chunks) in manifest.keyspaces {
        tracing::debug!(
            keyspace_name,
            num_parts = chunks.len(),
            "deserializing a keyspace"
        );
        let keyspace = db.keyspace(&keyspace_name, KeyspaceCreateOptions::default)?;
        deserialize_keyspace(&mut z, &keyspace, chunks)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use fjall::{Database, KeyspaceCreateOptions, Slice};

    use crate::core::cluster::serialized_state_machine::{load_from_file, serialize_to_file};

    #[test]
    fn test_serialize_to_file_round_trip() -> anyhow::Result<()> {
        let workdir = tempfile::tempdir()?;
        let mut db_path = workdir.path().to_path_buf();
        db_path.push("db/");

        let db = Database::builder(db_path).open()?;
        let ks1 = db.keyspace("keyspace1", KeyspaceCreateOptions::default)?;
        ks1.insert("key1", b"value1")?;
        ks1.insert("key2", b"\x00\x00\x00")?;
        ks1.insert("key3", b"")?;
        let ks2 = db.keyspace("keyspace2", KeyspaceCreateOptions::default)?;
        ks2.insert("key4", b"value4")?;
        db.persist(fjall::PersistMode::SyncAll)?;

        let snapshot = db.snapshot();

        let mut cursor = Cursor::new(vec![]);

        serialize_to_file(
            db,
            snapshot,
            vec!["keyspace1".to_owned(), "keyspace2".to_owned()],
            &mut cursor,
        )?;

        let out = cursor.into_inner();

        assert_eq!(&out[..8], b"COYOTE01");

        let mut db2_path = workdir.path().to_path_buf();
        db2_path.push("db_loaded/");
        let db2 = Database::builder(db2_path).open()?;

        let mut cursor = Cursor::new(out);

        load_from_file(&db2, &mut cursor)?;

        let found_keyspaces = db2
            .list_keyspace_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(&found_keyspaces, &["keyspace1", "keyspace2"]);

        let keyspace1 = db2.keyspace("keyspace1", KeyspaceCreateOptions::default)?;
        assert_eq!(
            keyspace1.get("key1")?.as_ref(),
            Some(&Slice::new(b"value1"))
        );
        assert_eq!(
            keyspace1.get("key2")?.as_ref(),
            Some(&Slice::new(b"\x00\x00\x00"))
        );
        assert_eq!(keyspace1.get("key3")?.as_ref(), Some(&Slice::new(b"")));
        assert_eq!(keyspace1.get("key4")?.as_ref(), None);

        let keyspace2 = db2.keyspace("keyspace2", KeyspaceCreateOptions::default)?;
        assert_eq!(
            keyspace2.get("key4")?.as_ref(),
            Some(&Slice::new(b"value4"))
        );

        Ok(())
    }
}
