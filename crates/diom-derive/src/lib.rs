#![warn(clippy::str_to_string)]

use proc_macro::TokenStream;
use syn::{DeriveInput, ItemFn, parse_macro_input};

mod aide;
mod dumpable_config;
mod env_overridable;
mod persistable_value;
mod utils;

use crate::{
    dumpable_config::derive_dumpable_config, env_overridable::derive_env_overridable,
    persistable_value::derive_persistable_value,
};
mod fjall_key;
mod fjall_key_component;

use self::aide::{AideAnnotateArgumentList, expand_aide_annotate};

#[proc_macro_derive(FjallKey, attributes(table_key, key))]
pub fn derive_fjall_key_able(input: TokenStream) -> TokenStream {
    fjall_key::derive(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(FjallKeyComponent)]
pub fn derive_key_component(input: TokenStream) -> TokenStream {
    fjall_key_component::derive(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Does nothing.
///
/// Replaces the real JsonSchema derive macro if the openapi Cargo feature is not enabled.
#[proc_macro_derive(JsonSchemaDummyDerive, attributes(schemars))]
pub fn dummy_derive_json_schema(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
/// Generate an aide operation transform for an axum handler function.
///
/// The generated function has the same name as the handler, suffixed with
/// `_operation`. It automatically sets the operation ID, summary and
/// description.
///
/// # Example
/// ```ignore
/// /// This is foo!
/// #[aide_annotate]
/// fn foo() {
/// }
///
/// /// This is bar, with a custom op ID and summary
/// #[aide_annotate(op_id = "custom_id", op_summary = "Bar Operation!")]
/// fn bar() {
/// }
/// ```
pub fn aide_annotate(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args with AideAnnotateArgumentList::parse_terminated);
    let item = parse_macro_input!(input as ItemFn);

    expand_aide_annotate(args, item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(EnvOverridable, attributes(env_overridable))]
pub fn macro_derive_env_overridable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_env_overridable(input).into()
}

#[proc_macro_derive(DumpableConfig, attributes(dumpable_config))]
pub fn macro_derive_dumpable_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_dumpable_config(input).into()
}

#[proc_macro_derive(PersistableValue)]
pub fn macro_derive_persistable_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_persistable_value(input).into()
}
