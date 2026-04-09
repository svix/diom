use quote::quote;
use syn::{
    DeriveInput, GenericParam, Generics, Ident, ItemFn, LitStr, parenthesized, parse_macro_input,
    parse_quote,
};

mod aide;

use self::aide::{AideAnnotateArgumentList, expand_aide_annotate};

#[proc_macro_derive(ModelOut)]
pub fn derive_model_out(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: BaseId` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics crate::v1::utils::ModelOut for #name #ty_generics #where_clause {
            fn id_copy(&self) -> String {
                self.id.0.clone()
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

/// Does nothing.
///
/// Replaces the real JsonSchema derive macro if the openapi Cargo feature is not enabled.
#[proc_macro_derive(JsonSchemaDummyDerive, attributes(schemars, validate))]
pub fn dummy_derive_json_schema(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::new()
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
pub fn aide_annotate(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args with AideAnnotateArgumentList::parse_terminated);
    let item = parse_macro_input!(input as ItemFn);

    expand_aide_annotate(args, item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

// Add a bound `T: HeapSize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(heapsize::HeapSize));
        }
    }
    generics
}

fn ungroup(mut ty: &syn::Type) -> &syn::Type {
    while let syn::Type::Group(group) = ty {
        ty = &group.elem;
    }
    ty
}

/// Determine if the given type is an option and, if so, extract its inner type name
fn as_ty_option(ty: &syn::Type) -> Option<&syn::Type> {
    let syn::Type::Path(ty) = ungroup(ty) else {
        return None;
    };
    let seg = ty.path.segments.last()?;
    let syn::PathArguments::AngleBracketed(bracketed) = &seg.arguments else {
        return None;
    };
    if seg.ident != "Option" || bracketed.args.len() != 1 {
        return None;
    }
    let Some(syn::GenericArgument::Type(arg)) = bracketed.args.get(0) else {
        return None;
    };
    Some(arg)
}

/// Determine whether the final segment of the given type has the given name
fn is_ty_name(name: &str, ty: &syn::Type) -> bool {
    let syn::Type::Path(ty) = ungroup(ty) else {
        return false;
    };
    ty.path
        .segments
        .last()
        .map(|s| s.ident == name)
        .unwrap_or(false)
}

struct EoField {
    variable: String,
    field: Ident,
    is_optional: bool,
    is_vec: bool,
    is_duration: bool,
    docstring: Vec<String>,
    nest: Option<(String, syn::Type)>,
}

impl EoField {
    fn parse(field: &syn::Field) -> Result<Option<Self>, syn::Error> {
        let mut render = true;
        let Some(ident) = &field.ident else {
            return Ok(None);
        };
        let mut nest = None;

        let is_vec;
        let is_duration;
        let is_optional;
        let mut docstring = vec![];

        if let Some(option_inner) = as_ty_option(&field.ty) {
            is_optional = true;
            is_duration = is_ty_name("DurationMs", option_inner);
            is_vec = is_ty_name("Vec", option_inner);
        } else {
            is_optional = false;
            is_duration = is_ty_name("DurationMs", &field.ty);
            is_vec = is_ty_name("Vec", &field.ty);
        }

        let mut variable = ident.to_string().to_uppercase();
        if is_duration {
            variable.push_str("_MS");
        }

        for attr in &field.attrs {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(nv) = &attr.meta
                    && let syn::Expr::Lit(lit) = &nv.value
                    && let syn::Lit::Str(lit) = &lit.lit
                {
                    let value = lit.value();
                    docstring.push(value.trim_start().to_string())
                }
            } else if attr.path().is_ident("env_overridable") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        render = false;
                        Ok(())
                    } else if meta.path.is_ident("var") {
                        let content;
                        parenthesized!(content in meta.input);
                        let lit: LitStr = content.parse()?;
                        variable = lit.value();
                        Ok(())
                    } else if meta.path.is_ident("nest_with_prefix") {
                        let content;
                        parenthesized!(content in meta.input);
                        let lit: LitStr = content.parse()?;
                        let recursive_prefix = lit.value();
                        let ty = field.ty.clone();
                        nest = Some((recursive_prefix, ty));
                        Ok(())
                    } else {
                        Err(meta.error("unrecognized repr"))
                    }
                })?;
            }
        }
        if !render {
            return Ok(None);
        }

        Ok(Some(Self {
            field: ident.clone(),
            variable,
            nest,
            is_optional,
            is_vec,
            is_duration,
            docstring,
        }))
    }
}

#[proc_macro_derive(EnvOverridable, attributes(env_overridable))]
pub fn derive_env_overridable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    let syn::Data::Struct(obj) = input.data else {
        return quote! { compile_error!("This macro may only be applied to structs") }.into();
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut fields = Vec::with_capacity(obj.fields.len());
    let mut lists = Vec::with_capacity(obj.fields.len());
    for field in obj.fields.iter() {
        let parsed = match EoField::parse(field) {
            Ok(Some(field)) => field,
            Ok(None) => continue,
            Err(e) => return e.to_compile_error().into(),
        };
        let variable = parsed.variable;
        let field = parsed.field;

        let setter = if parsed.is_optional {
            quote! { Some(value) }
        } else {
            quote! { value }
        };

        let loader = if parsed.is_vec {
            quote! { crate::cfg::env_overridable::env_var_comma_separated }
        } else if parsed.is_duration {
            quote! { crate::cfg::env_overridable::env_var_ms }
        } else {
            quote! { crate::cfg::env_overridable::env_var }
        };

        let field_parser = if let Some((recurse_name, _recurse_ty)) = &parsed.nest {
            quote! {
                let name = format!("{}_{}", prefix, #recurse_name);
                self.#field.load_environment_with_prefix(name)?;
            }
        } else {
            quote! {
                let name = format!("{}_{}", prefix, #variable);

                if let Some(value) = #loader(name)? {
                    self.#field = #setter;
                }
            }
        };
        fields.push(field_parser);

        let lister = if let Some((recurse_name, recurse_ty)) = &parsed.nest {
            quote! {
                let name = format!("{}_{}", prefix, #recurse_name);

                variables.extend(#recurse_ty::list_environment_variables_with_prefix(name));
            }
        } else {
            let docstring = if parsed.docstring.is_empty() {
                quote! { None }
            } else {
                let ds = parsed.docstring.join("\n");
                let ds = ds.trim();
                quote! { Some(#ds) }
            };
            quote! {
                let name = if prefix.is_empty() {
                    #variable.to_string()
                } else {
                    format!("{}_{}", prefix, #variable)
                };
                let v = crate::cfg::env_overridable::Variable {
                    env_var: name,
                    docstring: #docstring
                };
                variables.push(v);
            }
        };
        lists.push(lister);
    }

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let expanded = quote! {
        impl #impl_generics crate::cfg::env_overridable::EnvOverridable for #name #ty_generics #where_clause {
            fn load_environment_with_prefix(&mut self, prefix: String) -> anyhow::Result<()> {
                #(#fields)*;
                Ok(())
            }

            fn list_environment_variables_with_prefix(prefix: String) -> Vec<crate::cfg::env_overridable::Variable> {
                let mut variables = vec![];
                #(#lists)*;
                variables
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}
