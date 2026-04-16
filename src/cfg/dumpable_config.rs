use std::io::Write;

use serde::Serialize;

pub(crate) trait DumpableConfig {
    fn dump_fields<W: Write>(&self, writer: &mut W, prefix: String) -> anyhow::Result<()>;

    fn dump_map<W: Write>(&self, writer: &mut W, prefix: String) -> anyhow::Result<()> {
        writeln!(writer, "[{prefix}]")?;
        self.dump_fields(writer, prefix)
    }
}

pub(crate) fn dump_field<T: Serialize + ?Sized>(
    name: &str,
    value: &T,
    writer: &mut impl Write,
) -> Result<(), anyhow::Error> {
    let mut buffer = String::new();
    let serialized = value.serialize(toml::ser::ValueSerializer::new(&mut buffer))?;
    writeln!(writer, "{name} = {serialized}")?;
    Ok(())
}

pub(crate) fn dump_optional_field<T: Serialize + ?Sized>(
    name: &str,
    value: Option<&T>,
    writer: &mut impl Write,
) -> anyhow::Result<()> {
    if let Some(value) = value {
        dump_field(name, value, writer)?;
    } else {
        writeln!(writer, "# {name} =")?;
    }

    Ok(())
}
