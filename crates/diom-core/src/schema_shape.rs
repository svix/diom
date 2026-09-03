//! Machine-readable shapes of persisted types, gathered at link time.
//!
//! Every `PersistableValue` and `PersistableVersioned` type registers a [`SchemaShape`] through
//! `inventory`. A collector renders all of them into a canonical snapshot which is committed to the
//! repo. CI regenerates the snapshot and fails when it differs, so any change to a stored struct or
//! operation surfaces in review instead of silently breaking on-disk or replicated data.

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

/// Renders every registered shape into a stable, sorted, human-readable snapshot.
///
/// The output is deterministic across runs (sorted by module path then type name), so it can be
/// committed and diffed by CI.
pub fn render_snapshot() -> String {
    let mut shapes: Vec<&SchemaShape> = inventory::iter::<SchemaShape>.into_iter().collect();
    shapes.sort_by_key(|s| (s.module_path, s.type_name));

    let mut out = String::new();
    for shape in shapes {
        out.push_str(&format!(
            "{}::{} ({})\n",
            shape.module_path, shape.type_name, shape.kind
        ));
        for member in shape.members {
            // A unit enum variant has no type, so render just its name (no trailing `: `).
            let mut line = format!("    {}", member.name);
            if !member.ty.is_empty() {
                line.push_str(&format!(": {}", tidy_type(member.ty)));
            }
            if member.since > 0 {
                line.push_str(&format!(" since={}", member.since));
            }
            if member.nested {
                line.push_str(" nested");
            }
            out.push_str(line.trim_end());
            out.push('\n');
        }
    }
    out
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
