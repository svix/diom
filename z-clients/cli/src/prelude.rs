/// A wrapper type that allows passing a string or a byte array as an argument
///
/// If the argument starts with `[` and ends with `]`, it will be parsed as a series
/// of decimal values (e.g., `[1, 2, 3]` will be encoded as 0x010203); otherwise, it will
/// be treated as a utf-8 string (e.g., `123` will be encoded as 0x495051).
#[derive(Debug, Clone)]
pub(crate) enum ByteString {
    String(String),
    Bytes(Vec<u8>),
}

impl std::str::FromStr for ByteString {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("[") && s.ends_with("]") {
            let bytes = s[1..s.len() - 1]
                .split(",")
                .map(|s| s.trim().parse::<u8>())
                .collect::<Result<Vec<u8>, _>>()?;
            Ok(Self::Bytes(bytes))
        } else {
            Ok(Self::String(s.to_owned()))
        }
    }
}

impl std::fmt::Display for ByteString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => s.fmt(f),
            Self::Bytes(b) => {
                write!(f, "[")?;
                for (i, item) in b.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{item}")?;
                    } else {
                        write!(f, ", {item}")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl From<ByteString> for Vec<u8> {
    fn from(value: ByteString) -> Self {
        match value {
            ByteString::String(s) => s.into_bytes(),
            ByteString::Bytes(v) => v,
        }
    }
}
