use std::{collections::HashMap, fmt, ops::Deref};

use schemars::{JsonSchema, json_schema};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use crate::PersistableValue;

/// String templating for user-configured values.
///
/// A [`Template`] is a user-facing string that may embed `${name}` placeholders, e.g.
/// `"api.example.com/${org_id}/x"`.
///
/// # Syntax
///
/// - `${name}` — a placeholder, replaced by the value of the variable `name` at apply-time.
///   A name is any non-empty run of characters other than `$`, `{`, and `}`.
/// - `$$` — an escaped literal `$` (so `$${x}` renders as the literal text `${x}`).
/// - A lone `$` not followed by `{` or `$` is ordinary text (`"Price is $5"` renders verbatim).
///
/// A template is malformed if there are:
///
/// - unterminated variables (e.g. `api.com/${org_id`)
/// - empty variables (e.g. `api.com/${}`)
/// - nested variables (e.g. `api.com/${${${${org_id}}}}`)
///
/// To use a template, it needs to be [compiled](Template::compile) once into a [`CompiledTemplate`] and
/// then [applied](CompiledTemplate::apply) against a map of variables to produce the final
/// string.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Template(String);

// `PersistableValue`'s derive references the `diom_core` crate by name, which does not resolve
// from inside `diom-core` itself, so the marker trait is implemented by hand here.
impl PersistableValue for Template {}

pub struct CompiledTemplate<'a>(Vec<Entry<'a>>);

enum Entry<'a> {
    /// Literal text, emitted verbatim. Always a slice of the source template (an escaped `$$`
    /// is represented as a `Const` pointing at a single `$` character).
    Const(&'a str),
    /// A `${name}` placeholder; carries the variable `name` (without the `${ }` delimiters).
    Variable(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub struct TemplateError(String);

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for TemplateError {}

impl Template {
    pub fn new(template: String) -> Result<Self, TemplateError> {
        parse(&template, true)?;
        Ok(Self(template))
    }

    pub fn compile(&self) -> CompiledTemplate<'_> {
        CompiledTemplate(parse(&self.0, false).expect("Template was validated on construction"))
    }
}

impl Deref for Template {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for Template {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Template {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(s).map_err(de::Error::custom)
    }
}

impl JsonSchema for Template {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        String::schema_name()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "example": "My name is ${name}!",
        })
    }
}

impl<'a> CompiledTemplate<'a> {
    pub fn apply(&self, vars: &HashMap<String, String>) -> String {
        let mut out = {
            let size_hint = self
                .0
                .iter()
                .map(|e| match e {
                    Entry::Const(s) => s.len(),
                    Entry::Variable(key) => vars.get(*key).map_or(0, |s| s.len()),
                })
                .sum();

            String::with_capacity(size_hint)
        };

        for entry in &self.0 {
            match entry {
                Entry::Const(text) => out.push_str(text),
                Entry::Variable(name) => {
                    if let Some(value) = vars.get(*name) {
                        out.push_str(value);
                    }
                }
            }
        }
        out
    }
}

fn parse(s: &str, dry_run: bool) -> Result<Vec<Entry<'_>>, TemplateError> {
    let mut entries = Vec::new();
    // Collects an entry unless we're only validating. `Vec::new` doesn't allocate until the
    // first push, so a dry run leaves `entries` empty and unallocated.
    let mut push = |entry| {
        if !dry_run {
            entries.push(entry);
        }
    };

    // Start of the current run of literal text not yet flushed into an `Entry::Const`.
    let mut const_start = 0;
    let mut chars = s.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        if c != '$' {
            continue;
        }

        if const_start < i {
            push(Entry::Const(&s[const_start..i]));
        }

        match chars.peek() {
            // `$$` — escaped literal `$`. Emit a single `$` (a slice of the second one).
            Some(&(j, '$')) => {
                push(Entry::Const(&s[j..j + 1]));
                chars.next();
                const_start = j + 1;
            }
            // `${name}` — a placeholder.
            Some(&(brace, '{')) => {
                chars.next();
                let name_start = brace + 1; // `{` is one byte
                let close = read_placeholder_close(&mut chars)?;
                if close == name_start {
                    return Err(TemplateError("empty variable name in `${}`".to_owned()));
                }
                push(Entry::Variable(&s[name_start..close]));
                const_start = close + 1; // `}` is one byte
            }
            // A lone `$` (before other text or at end of string) is literal.
            _ => {
                push(Entry::Const(&s[i..i + 1]));
                const_start = i + 1;
            }
        }
    }

    if const_start < s.len() {
        push(Entry::Const(&s[const_start..]));
    }

    Ok(entries)
}

/// Reads the body of a `${...}` placeholder from `chars` (positioned just after the `{`) and
/// returns the byte index of the closing `}`.
///
/// `$` and `{` inside the name signal malformed nesting and are rejected; every other character
/// is a valid name character.
fn read_placeholder_close(
    chars: &mut impl Iterator<Item = (usize, char)>,
) -> Result<usize, TemplateError> {
    loop {
        match chars.next() {
            None => {
                return Err(TemplateError(
                    "unterminated `${` placeholder: missing closing `}`".to_owned(),
                ));
            }
            Some((k, '}')) => return Ok(k),
            Some((_, ch @ ('$' | '{'))) => {
                return Err(TemplateError(format!(
                    "`{ch}` is not allowed in a variable name"
                )));
            }
            Some(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    fn render(template: &str, pairs: &[(&str, &str)]) -> String {
        Template::new(template.to_owned())
            .unwrap()
            .compile()
            .apply(&vars(pairs))
    }

    #[test]
    fn const_only() {
        assert_eq!(render("hello world", &[]), "hello world");
    }

    #[test]
    fn single_variable() {
        assert_eq!(render("a/${x}/b", &[("x", "1")]), "a/1/b");
    }

    #[test]
    fn multiple_and_adjacent_variables() {
        assert_eq!(
            render("api.example.com/${org_id}/x", &[("org_id", "1234")]),
            "api.example.com/1234/x"
        );
        assert_eq!(render("${a}${b}", &[("a", "1"), ("b", "2")]), "12");
    }

    #[test]
    fn missing_variable_renders_empty() {
        assert_eq!(render("a/${x}/b", &[]), "a//b");
    }

    #[test]
    fn escaped_dollar_is_literal() {
        assert_eq!(render("$${x}", &[("x", "nope")]), "${x}");
        assert_eq!(render("a$$b", &[]), "a$b");
    }

    #[test]
    fn lone_dollar_is_literal() {
        assert_eq!(render("cost $5", &[]), "cost $5");
        assert_eq!(render("ends with $", &[]), "ends with $");
    }

    #[test]
    fn rejects_unterminated_placeholder() {
        let err = Template::new("a/${org_id".to_owned()).unwrap_err();
        assert!(err.to_string().contains("unterminated"), "{err}");
    }

    #[test]
    fn rejects_empty_variable() {
        let err = Template::new("${}".to_owned()).unwrap_err();
        assert!(err.to_string().contains("empty variable name"), "{err}");
    }

    #[test]
    fn allows_unusual_variable_names() {
        // Any character other than `$`, `{`, `}` is a valid name character.
        assert_eq!(render("${org id}", &[("org id", "1")]), "1");
        assert_eq!(render("${org-id}", &[("org-id", "2")]), "2");
    }

    #[test]
    fn rejects_nested_placeholders() {
        // `$` and `{` are not allowed inside a name, so nested placeholders are rejected.
        for template in ["${ ${x} }", "${a{b}", "${a$b}"] {
            let err = Template::new(template.to_owned()).unwrap_err();
            assert!(err.to_string().contains("not allowed"), "{template}: {err}");
        }
    }
}
