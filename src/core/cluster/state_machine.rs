use std::{
    io::{ErrorKind, Seek, SeekFrom},
    path::{Path, PathBuf},
    pin::Pin,
    sync::{Arc, RwLock},
};

use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode, Readable};
use openraft::{
    EntryPayload, LogId, RaftSnapshotBuilder, RaftTypeConfig, Snapshot, SnapshotMeta,
    StoredMembership, storage::RaftStateMachine,
};
use serde::{Deserialize, Serialize};

use super::{
    errors::*,
    raft::{Node, TypeConfig},
    serialized_state_machine,
};
use crate::core::cluster::raft;

type NodeId = <TypeConfig as RaftTypeConfig>::NodeId;
type StorageError = openraft::StorageError<NodeId>;
type StorageResult<T> = Result<T, StorageError>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LastSnapshot {
    meta: SnapshotMeta<NodeId, Node>,
    path: PathBuf,
}

#[derive(Clone)]
pub struct Store {
    db: Database,
    snapshot_directory: PathBuf,
    meta_keyspace: Keyspace,
    snapshot_idx: u64,
    last_applied_log_id: Option<LogId<NodeId>>,
    last_membership: StoredMembership<NodeId, Node>,
    last_snapshot: Arc<RwLock<Option<LastSnapshot>>>,
}

trait SnapshotIdx {
    fn snapshot_idx(&self) -> u64;
}

impl SnapshotIdx for SnapshotMeta<NodeId, Node> {
    fn snapshot_idx(&self) -> u64 {
        // TODO: fix these unwraps
        self.snapshot_id
            .split('-')
            .next_back()
            .unwrap()
            .parse()
            .unwrap()
    }
}

const METADATA_KEYSPACE: &str = "_raft_metadata";

impl Store {
    pub async fn new(db: Database, snapshot_directory: PathBuf) -> anyhow::Result<Self> {
        if let Err(e) = tokio::fs::create_dir_all(&snapshot_directory).await
            && e.kind() != ErrorKind::AlreadyExists
        {
            return Err(e.into());
        }
        let meta_keyspace = db.keyspace(METADATA_KEYSPACE, KeyspaceCreateOptions::default)?;
        let mut this = Self {
            db,
            snapshot_directory,
            meta_keyspace,
            last_snapshot: Arc::new(RwLock::new(None)),
            snapshot_idx: 0,
            last_applied_log_id: None,
            last_membership: Default::default(),
        };
        this.load_information().await?;
        Ok(this)
    }

    #[tracing::instrument(skip(self))]
    async fn load_information(&mut self) -> anyhow::Result<()> {
        let db = self.db.clone();
        let keyspace = self.meta_keyspace.clone();

        let (last_applied_log_id, last_membership, last_snapshot) =
            tokio::task::spawn_blocking(move || -> anyhow::Result<(_, _, _)> {
                let snapshot = db.snapshot();
                let last_applied_log_id = if let Some(raw_last_applied_log_id) =
                    snapshot.get(&keyspace, "last_applied_log_id")?
                {
                    let last_applied_log_id = rmp_serde::from_slice(&raw_last_applied_log_id)?;
                    Some(last_applied_log_id)
                } else {
                    None
                };
                let last_membership = if let Some(raw_last_membership) =
                    snapshot.get(&keyspace, "last_membership")?
                {
                    rmp_serde::from_slice(&raw_last_membership)?
                } else {
                    tracing::trace!("found no last_membership in database!");
                    Default::default()
                };

                let last_snapshot =
                    if let Some(raw_snapshot_data) = snapshot.get(&keyspace, "last_snapshot")? {
                        let s: LastSnapshot = rmp_serde::from_slice(&raw_snapshot_data)?;
                        Some(s)
                    } else {
                        None
                    };

                Ok((last_applied_log_id, last_membership, last_snapshot))
            })
            .await??;
        self.last_applied_log_id = last_applied_log_id;
        self.last_membership = last_membership;
        self.snapshot_idx = last_snapshot
            .as_ref()
            .map(|s| s.meta.snapshot_idx())
            .unwrap_or(0);
        *self.last_snapshot.write().unwrap() = last_snapshot;
        Ok(())
    }

    async fn set_last_snapshot_(
        &mut self,
        meta: SnapshotMeta<NodeId, Node>,
        snapshot_path: PathBuf,
    ) -> anyhow::Result<()> {
        let db = self.db.clone();
        let keyspace = self.meta_keyspace.clone();
        self.snapshot_idx = std::cmp::max(self.snapshot_idx + 1, meta.snapshot_idx());
        let data = LastSnapshot {
            meta,
            path: snapshot_path,
        };
        tracing::trace!(last_snapshot=?data, "setting last_snapshot");
        let serialized = rmp_serde::to_vec(&data)?;
        tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
            keyspace.insert("last_snapshot", serialized)?;
            db.persist(fjall::PersistMode::SyncAll)?;
            Ok(())
        })
        .await??;
        *self.last_snapshot.write().unwrap() = Some(data);
        Ok(())
    }

    async fn record_ids_(&mut self) -> anyhow::Result<()> {
        tracing::trace!(
            last_applied_log_id=?self.last_applied_log_id,
            last_membership=?self.last_membership,
            "storing id values");
        let mut tx = self.db.batch().durability(Some(PersistMode::Buffer));
        let keyspace = self.meta_keyspace.clone();
        let last_applied_log_id = rmp_serde::encode::to_vec(&self.last_applied_log_id)?;
        let last_membership = rmp_serde::encode::to_vec(&self.last_membership)?;
        tokio::task::spawn_blocking(move || {
            tx.insert(&keyspace, "last_applied_log_id", last_applied_log_id);
            tx.insert(&keyspace, "last_membership", last_membership);
            tx.commit()?;
            Ok(())
        })
        .await?
    }

    async fn delete_unused_snapshots(&self, keep_path: &Path) -> anyhow::Result<()> {
        tracing::debug!("cleaning up unused snapshots");
        let mut dents = tokio::fs::read_dir(&self.snapshot_directory).await?;
        while let Some(dent) = dents.next_entry().await? {
            if let Some(preserve_path) = keep_path.file_name() {
                if dent.file_name() == preserve_path {
                    tracing::trace!(filename=?dent.file_name(), "preserving used snapshot");
                    continue;
                }
            } else {
                tracing::warn!(path=?keep_path, "very weird snapshot path");
            }
            tracing::debug!(filename=?dent.file_name(), "deleting unused snapshot");
            tokio::fs::remove_file(dent.path()).await?;
        }
        Ok(())
    }

    async fn install_snapshot_(
        &mut self,
        meta: &openraft::SnapshotMeta<NodeId, Node>,
        snapshot: Box<StoredSnapshot>,
    ) -> anyhow::Result<()> {
        tracing::debug!("starting snapshot installation");
        let mut f = snapshot.file.into_std().await;
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || serialized_state_machine::load_from_file(&db, &mut f))
            .await??;
        self.last_applied_log_id = meta.last_log_id;
        self.last_membership = meta.last_membership.clone();
        self.record_ids_().await?;
        self.delete_unused_snapshots(snapshot.path.as_path())
            .await?;
        self.set_last_snapshot_(meta.clone(), snapshot.path).await?;
        Ok(())
    }
}

// Wrapper around a snapshot that has been written to disk and has both a filename
// and a concrete File
pub struct StoredSnapshot {
    file: tokio::fs::File,
    path: PathBuf,
}

impl std::fmt::Debug for StoredSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StoredSnapshot {{ path: {:?} }}", self.path)
    }
}

impl tokio::io::AsyncWrite for StoredSnapshot {
    fn is_write_vectored(&self) -> bool {
        self.file.is_write_vectored()
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        Pin::new(&mut self.file).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        Pin::new(&mut self.file).poll_shutdown(cx)
    }

    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        Pin::new(&mut self.file).poll_write(cx, buf)
    }

    fn poll_write_vectored(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> std::task::Poll<std::io::Result<usize>> {
        Pin::new(&mut self.file).poll_write_vectored(cx, bufs)
    }
}

impl tokio::io::AsyncRead for StoredSnapshot {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        Pin::new(&mut self.file).poll_read(cx, buf)
    }
}

impl tokio::io::AsyncSeek for StoredSnapshot {
    fn poll_complete(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<u64>> {
        Pin::new(&mut self.file).poll_complete(cx)
    }

    fn start_seek(mut self: std::pin::Pin<&mut Self>, position: SeekFrom) -> std::io::Result<()> {
        Pin::new(&mut self.file).start_seek(position)
    }
}

impl StoredSnapshot {
    async fn new(
        metadata: &SnapshotMeta<NodeId, Node>,
        directory: &Path,
        database: Database,
        keyspaces: Vec<String>,
    ) -> anyhow::Result<Self> {
        let file_name = format!("coyote-{}", metadata.snapshot_id);
        let path = directory.with_file_name(file_name);
        let path_c = path.clone();
        let file = tokio::task::spawn_blocking(move || -> anyhow::Result<std::fs::File> {
            let snapshot = database.snapshot();
            tracing::info!(path=%path_c.display(), "writing snapshot");
            let mut f = std::fs::File::create(path_c)?;
            serialized_state_machine::serialize_to_file(database, snapshot, keyspaces, &mut f)?;
            f.seek(SeekFrom::Start(0))?;
            Ok(f)
        })
        .await??;
        let file = tokio::fs::File::from_std(file);
        Ok(Self { file, path })
    }
}

impl RaftSnapshotBuilder<TypeConfig> for Store {
    async fn build_snapshot(&mut self) -> StorageResult<openraft::Snapshot<TypeConfig>> {
        let last_log_id = self.last_applied_log_id;
        let last_membership = self.last_membership.clone();

        let snapshot_id = if let Some(last) = last_log_id {
            format!("{}-{}-{}", last.leader_id, last.index, self.snapshot_idx)
        } else {
            format!("x-x-{}", self.snapshot_idx)
        };

        let meta = SnapshotMeta {
            last_log_id,
            last_membership,
            snapshot_id,
        };

        let keyspaces = self
            .db
            .list_keyspace_names()
            .into_iter()
            .filter(|s| s.as_bytes() != METADATA_KEYSPACE.as_bytes())
            .map(|s| s.to_string())
            .collect();

        let snapshot =
            StoredSnapshot::new(&meta, &self.snapshot_directory, self.db.clone(), keyspaces)
                .await
                .map_err(write_snapshot_err)?;

        self.set_last_snapshot_(meta.clone(), snapshot.path.clone())
            .await
            .map_err(write_snapshot_err)?;

        let snapshot = Box::new(snapshot);

        Ok(Snapshot { meta, snapshot })
    }
}

impl RaftStateMachine<TypeConfig> for Store {
    type SnapshotBuilder = Self;

    async fn applied_state(
        &mut self,
    ) -> StorageResult<(Option<LogId<NodeId>>, StoredMembership<NodeId, Node>)> {
        Ok((self.last_applied_log_id, self.last_membership.clone()))
    }

    async fn apply<I>(&mut self, entries: I) -> StorageResult<Vec<raft::Response>>
    where
        I: IntoIterator<Item = <TypeConfig as RaftTypeConfig>::Entry> + openraft::OptionalSend,
        I::IntoIter: openraft::OptionalSend,
    {
        let mut replies = vec![];
        let mut changed_log_id = false;
        let mut changed_membership = false;
        for item in entries {
            self.last_applied_log_id = Some(item.log_id);
            changed_log_id = true;

            match item.payload {
                EntryPayload::Blank => {
                    tracing::trace!("heartbeat");
                    replies.push(raft::Response::blank())
                }
                EntryPayload::Normal(req) => {
                    // TODO actually apply something
                    tracing::trace!(log_id=?item.log_id, request=?req, "applying user request");
                }
                EntryPayload::Membership(last_membership) => {
                    tracing::trace!("changing cluster membership");
                    self.last_membership =
                        StoredMembership::new(Some(item.log_id), last_membership);
                    changed_membership = true;
                    replies.push(raft::Response::blank())
                }
            }
        }
        if changed_log_id || changed_membership {
            self.record_ids_().await.map_err(write_err)?;
        }
        Ok(replies)
    }

    async fn begin_receiving_snapshot(
        &mut self,
    ) -> StorageResult<Box<<TypeConfig as RaftTypeConfig>::SnapshotData>> {
        let path = self
            .snapshot_directory
            .with_file_name(format!("coyote-incoming-snapshot-{}", self.snapshot_idx));
        self.snapshot_idx += 1;
        let f = tokio::fs::File::create_new(path.clone())
            .await
            .map_err(|e| write_snapshot_err(&e))?;
        Ok(Box::new(StoredSnapshot { path, file: f }))
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        self.snapshot_idx += 1;
        self.clone()
    }

    #[tracing::instrument(skip(self))]
    async fn get_current_snapshot(
        &mut self,
    ) -> StorageResult<Option<openraft::Snapshot<TypeConfig>>> {
        // clone to avoid holding a lock over an await point
        let last_snapshot = self.last_snapshot.read().unwrap().clone();
        if let Some(last_snapshot) = last_snapshot {
            tracing::trace!(?last_snapshot, "found last_snapshot");
            let f = tokio::fs::File::open(&last_snapshot.path)
                .await
                .map_err(|e| read_snapshot_err(&e))?;
            Ok(Some(Snapshot {
                meta: last_snapshot.meta.clone(),
                snapshot: Box::new(StoredSnapshot {
                    file: f,
                    path: last_snapshot.path.clone(),
                }),
            }))
        } else {
            tracing::trace!("found no last_snapshot");
            Ok(None)
        }
    }

    #[tracing::instrument(skip(self, meta))]
    async fn install_snapshot(
        &mut self,
        meta: &openraft::SnapshotMeta<NodeId, Node>,
        snapshot: Box<StoredSnapshot>,
    ) -> StorageResult<()> {
        self.install_snapshot_(meta, snapshot)
            .await
            .map_err(read_snapshot_err)
    }
}
