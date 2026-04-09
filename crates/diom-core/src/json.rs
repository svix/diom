use std::io;

use serde::Serialize;

struct CompactByteStrFormatter;

impl serde_json::ser::Formatter for CompactByteStrFormatter {
    fn write_byte_array<W>(&mut self, writer: &mut W, value: &[u8]) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.begin_string(writer)?;
        let mut buf = Vec::new();
        v_jsonescape::b_escape(value, &mut buf);
        writer.write_all(&buf)?;
        self.end_string(writer)
    }
}

/// `serde_json::to_writer`, but writing bytes as escaped strings.
pub fn to_writer(writer: impl io::Write, value: &impl Serialize) -> serde_json::Result<()> {
    let mut serializer = serde_json::Serializer::with_formatter(writer, CompactByteStrFormatter);
    value.serialize(&mut serializer)
}

/// `serde_json::to_vec`, but writing bytes as escaped strings.
pub fn to_vec(value: &impl Serialize) -> serde_json::Result<Vec<u8>> {
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}
