pub(crate) trait DumpableConfig {
    fn dump_fields<W: std::io::Write>(&self, writer: &mut W, prefix: String) -> anyhow::Result<()>;

    fn dump_map<W: std::io::Write>(&self, writer: &mut W, prefix: String) -> anyhow::Result<()> {
        write!(writer, "\n[{prefix}]\n")?;
        self.dump_fields(writer, prefix)
    }
}

impl<T: DumpableConfig> DumpableConfig for Option<T> {
    fn dump_fields<W: std::io::Write>(&self, writer: &mut W, prefix: String) -> anyhow::Result<()> {
        if let Some(values) = self {
            values.dump_fields(writer, prefix)
        } else {
            Ok(())
        }
    }

    fn dump_map<W: std::io::Write>(&self, writer: &mut W, prefix: String) -> anyhow::Result<()> {
        if let Some(values) = self {
            values.dump_map(writer, prefix)
        } else {
            Ok(())
        }
    }
}
