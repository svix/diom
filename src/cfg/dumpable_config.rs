pub(crate) trait DumpableConfig {
    fn dump_fields<W: std::io::Write>(&self, writer: &mut W, prefix: String) -> anyhow::Result<()>;

    fn dump_map<W: std::io::Write>(&self, writer: &mut W, prefix: String) -> anyhow::Result<()> {
        writeln!(writer, "[{prefix}]")?;
        self.dump_fields(writer, prefix)
    }
}
