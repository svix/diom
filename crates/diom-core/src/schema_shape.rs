//! Machine-readable shapes of persisted types, gathered at link time.
//!
//! Every `PersistableValue` and `PersistableVersioned` type registers a [`SchemaShape`] through
//! `inventory`. A collector renders all of them into a canonical snapshot which is committed to the
//! repo. CI regenerates the snapshot and fails when it differs, so any change to a stored struct or
//! operation surfaces in review instead of silently breaking on-disk or replicated data.

use serde::Serialize;

/// One field of a struct, or one variant of an enum.
///
/// `since` and `nested` only carry meaning for `PersistableVersioned` types. Plain
/// `PersistableValue` members report `since` of 0 and `nested` of false.
pub struct MemberShape {
    pub name: &'static str,
    /// The member type rendered as source tokens, e.g. `Option < u32 >`.
    pub ty: &'static str,
    pub since: u32,
    pub nested: bool,
}

/// The registered shape of one persisted struct or enum.
pub struct SchemaShape {
    /// Module path where the type is declared, used to disambiguate equally named types.
    pub module_path: &'static str,
    pub type_name: &'static str,
    /// `"versioned"`, `"value-struct"`, or `"value-enum"`.
    pub kind: &'static str,
    pub members: &'static [MemberShape],
}

inventory::collect!(SchemaShape);

/// Serializable view of a registered type, used only to render the snapshot.
///
/// Mirrors [`SchemaShape`] but owns a tidied `ty` and drops default `since`/`nested` so the YAML
/// stays terse. The registration types stay plain so the derive never has to depend on serde.
#[derive(Serialize)]
struct TypeEntry {
    module_path: &'static str,
    type_name: &'static str,
    kind: &'static str,
    members: Vec<MemberEntry>,
}

#[derive(Serialize)]
struct MemberEntry {
    name: &'static str,
    /// A unit enum variant has no type, so it is omitted rather than written as an empty string.
    #[serde(skip_serializing_if = "String::is_empty")]
    ty: String,
    #[serde(skip_serializing_if = "is_zero")]
    since: u32,
    #[serde(skip_serializing_if = "is_false")]
    nested: bool,
}

fn is_zero(n: &u32) -> bool {
    *n == 0
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// Renders every registered shape into a stable, sorted YAML snapshot.
///
/// The output is deterministic across runs (sorted by module path then type name), so it can be
/// committed and diffed by CI.
pub fn render_snapshot() -> String {
    let mut shapes: Vec<&SchemaShape> = inventory::iter::<SchemaShape>.into_iter().collect();
    shapes.sort_by_key(|s| (s.module_path, s.type_name));

    let entries: Vec<TypeEntry> = shapes
        .into_iter()
        .map(|shape| TypeEntry {
            module_path: shape.module_path,
            type_name: shape.type_name,
            kind: shape.kind,
            members: shape
                .members
                .iter()
                .map(|member| MemberEntry {
                    name: member.name,
                    ty: tidy_type(member.ty),
                    since: member.since,
                    nested: member.nested,
                })
                .collect(),
        })
        .collect();

    let mut yaml = serde_saphyr::to_string(&entries).expect("serialize schema snapshot to YAML");
    if !yaml.ends_with('\n') {
        yaml.push('\n');
    }
    yaml
}

/// Cleans up nonsense like `Option < u64 >` to `Option<u64>`
fn tidy_type(spaced: &str) -> String {
    const NO_SPACE_AFTER: &[char] = &['<', '(', '[', '&', ':'];
    const NO_SPACE_BEFORE: &[char] = &['<', '>', '(', ')', '[', ']', ',', ';', ':'];

    let mut out = String::with_capacity(spaced.len());
    let mut chars = spaced.chars().peekable();
    while let Some(c) = chars.next() {
        if c == ' ' {
            let after_opener = out
                .chars()
                .last()
                .is_some_and(|p| NO_SPACE_AFTER.contains(&p));
            let before_closer = chars.peek().is_some_and(|n| NO_SPACE_BEFORE.contains(n));
            if after_opener || before_closer {
                continue;
            }
        }
        out.push(c);
    }
    out
}
