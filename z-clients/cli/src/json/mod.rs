use std::{io::Read, str::FromStr};

use anyhow::{Context, Error, Result};
use serde::{Serialize, de::DeserializeOwned};

mod format;

use self::format::PrettyFormatter;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct JsonOf<T>(T);

impl<T: DeserializeOwned> FromStr for JsonOf<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = if s == "-" {
            let mut stdin = std::io::stdin().lock();
            let mut input = String::new();
            stdin
                .read_to_string(&mut input)
                .context("Error reading stdin for '-' argument")?;
            input
        } else {
            s.to_owned()
        };

        Ok(JsonOf(serde_json::from_str(&strip_json_comments(&input))?))
    }
}

/// Removes `//` line comments and `/* */` block comments so a JSONC body can be
/// parsed as strict JSON.
///
/// Comment markers inside string literals are left untouched, and newlines
/// inside comments are preserved so parse-error line numbers still line up with
/// the original input.
fn strip_json_comments(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_string = false;
    let mut escaped = false;

    while let Some(c) = chars.next() {
        if in_string {
            out.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }

        match c {
            '"' => {
                in_string = true;
                out.push(c);
            }
            // Line comment: drop everything up to (but not including) the line
            // ending, which may be LF, CRLF or a lone CR.
            '/' if chars.peek() == Some(&'/') => {
                chars.next();
                while chars
                    .peek()
                    .is_some_and(|&next| !matches!(next, '\n' | '\r'))
                {
                    chars.next();
                }
            }
            // Block comment: replace with a single space (a comment acts as a
            // token separator, like whitespace), keeping newlines.
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                out.push(' ');
                while let Some(inner) = chars.next() {
                    if inner == '\n' {
                        out.push('\n');
                    } else if inner == '*' && chars.peek() == Some(&'/') {
                        chars.next();
                        break;
                    }
                }
            }
            _ => out.push(c),
        }
    }

    out
}

impl<T> JsonOf<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

pub fn print_json_output<T>(val: &T) -> Result<()>
where
    T: Serialize,
{
    let mut stdout = std::io::stdout().lock();
    let mut serializer =
        serde_json::Serializer::with_formatter(&mut stdout, PrettyFormatter::default());
    match val.serialize(&mut serializer) {
        Ok(_) => Ok(()),
        Err(e) if matches!(e.io_error_kind(), Some(std::io::ErrorKind::BrokenPipe)) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Pair {
        a: u32,
        b: u32,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Note {
        url: String,
        note: String,
    }

    fn parse<T: DeserializeOwned>(s: &str) -> T {
        JsonOf::<T>::from_str(s).unwrap().into_inner()
    }

    #[test]
    fn plain_json_is_unchanged() {
        let input = r#"{"a": 1, "b": 2}"#;
        assert_eq!(strip_json_comments(input), input);
        assert_eq!(parse::<Pair>(input), Pair { a: 1, b: 2 });
    }

    #[test]
    fn strips_line_comments() {
        let input = "{\n  \"a\": 1 // trailing\n  ,\"b\": 2 // another\n}";
        assert_eq!(parse::<Pair>(input), Pair { a: 1, b: 2 });
    }

    #[test]
    fn strips_line_comments_with_carriage_returns() {
        let crlf = "{\r\n  \"a\": 1, // trailing\r\n  \"b\": 2\r\n}";
        let cr = "{\r  \"a\": 1, // trailing\r  \"b\": 2\r}";
        assert_eq!(parse::<Pair>(crlf), Pair { a: 1, b: 2 });
        assert_eq!(parse::<Pair>(cr), Pair { a: 1, b: 2 });
    }

    #[test]
    fn strips_block_comments() {
        let input = "{ /* leading */ \"a\": 1, \"b\": /* inline */ 2 }";
        assert_eq!(parse::<Pair>(input), Pair { a: 1, b: 2 });
    }

    #[test]
    fn block_comments_separate_tokens() {
        // A comment is a token separator, so this must not turn into `[60]`.
        assert_eq!(strip_json_comments("[6/*c*/0]"), "[6 0]");
        JsonOf::<Vec<u32>>::from_str("[6/*c*/0]").unwrap_err();
    }

    #[test]
    fn keeps_comment_markers_inside_strings() {
        let input = r#"{"url": "https://example.com", "note": "a /* b */ c"}"#;
        assert_eq!(strip_json_comments(input), input);
        assert_eq!(
            parse::<Note>(input),
            Note {
                url: "https://example.com".to_owned(),
                note: "a /* b */ c".to_owned(),
            },
        );
    }

    #[test]
    fn handles_escaped_quote_before_comment() {
        let input = r#"{"a": "b\"// not a comment"}"#;
        assert_eq!(strip_json_comments(input), input);
    }

    #[test]
    fn preserves_line_numbers_in_block_comments() {
        let input = "{\n/* one\ntwo */\n\"a\": nope\n}";
        // Error should point at line 4, where the invalid token actually is.
        let err = JsonOf::<Pair>::from_str(input).unwrap_err();
        assert!(err.to_string().contains("line 4"), "got: {err}");
    }
}
