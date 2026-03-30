use quote::quote;
use syn::{
    DeriveInput, GenericParam, Generics, Ident, ItemFn, LitStr, parenthesized, parse_macro_input,
    parse_quote, spanned::Spanned,
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

fn is_ty_option(ty: &syn::Type) -> Option<&syn::Type> {
    let path = match ungroup(ty) {
        syn::Type::Path(ty) => &ty.path,
        _ => {
            return None;
        }
    };
    let seg = path.segments.last()?;
    let args = match &seg.arguments {
        syn::PathArguments::AngleBracketed(bracketed) => &bracketed.args,
        _ => {
            return None;
        }
    };
    if seg.ident == "Option" && args.len() == 1 {
        let Some(syn::GenericArgument::Type(arg)) = args.get(0) else {
            return None;
        };
        Some(arg)
    } else {
        None
    }
}

fn is_ty_vec(ty: &syn::Type) -> bool {
    let path = match ungroup(ty) {
        syn::Type::Path(ty) => &ty.path,
        _ => {
            return false;
        }
    };
    let Some(seg) = path.segments.last() else {
        return false;
    };
    let args = match &seg.arguments {
        syn::PathArguments::AngleBracketed(bracketed) => &bracketed.args,
        _ => {
            return false;
        }
    };
    seg.ident == "Vec" && args.len() == 1
}

fn is_ty_duration(ty: &syn::Type) -> bool {
    let path = match ungroup(ty) {
        syn::Type::Path(ty) => &ty.path,
        _ => {
            return false;
        }
    };
    let Some(seg) = path.segments.last() else {
        return false;
    };
    seg.ident == "Duration"
}

struct EOField {
    variable: String,
    field: Ident,
    is_optional: bool,
    is_vec: bool,
    is_duration: bool,
    docstring: Vec<String>,
    nest: Option<(String, syn::Type)>,
}

impl EOField {
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

        if let Some(option_inner) = is_ty_option(&field.ty) {
            is_optional = true;
            is_duration = is_ty_duration(option_inner);
            is_vec = is_ty_vec(option_inner);
        } else {
            is_optional = false;
            is_duration = is_ty_duration(&field.ty);
            is_vec = is_ty_vec(&field.ty);
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
        return proc_macro::TokenStream::from(
            syn::Error::new(
                input.ident.span(),
                "This macro may only be applied to structs".to_string(),
            )
            .to_compile_error(),
        );
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut fields = Vec::with_capacity(obj.fields.len());
    let mut lists = Vec::with_capacity(obj.fields.len());
    for field in obj.fields.iter() {
        let parsed = match EOField::parse(field) {
            Ok(Some(field)) => field,
            Ok(None) => continue,
            Err(e) => {
                return proc_macro::TokenStream::from(
                    syn::Error::new(
                        field.span(),
                        format!("Invalid field while parsing structure: {e}"),
                    )
                    .to_compile_error(),
                );
            }
        };
        let variable = parsed.variable;
        let field = parsed.field;

        let setter = if parsed.is_optional {
            quote! { self.#field = Some(value) }
        } else {
            quote! { self.#field = value }
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
                let name = if prefix.is_empty() {
                    std::borrow::Cow::Borrowed(#recurse_name)
                } else {
                    std::borrow::Cow::Owned(format!("{}_{}", prefix, #recurse_name))
                };

                self.#field.load_environment_with_prefix(name)?;
            }
        } else {
            quote! {
                let name = if prefix.is_empty() {
                    std::borrow::Cow::Borrowed(#variable)
                } else {
                    std::borrow::Cow::Owned(format!("{}_{}", prefix, #variable))
                };

                if let Some(value) = #loader(name)? {
                    #setter;
                }
            }
        };
        fields.push(field_parser);

        let lister = if let Some((recurse_name, recurse_ty)) = &parsed.nest {
            quote! {
                let name = if prefix.is_empty() {
                    std::borrow::Cow::Borrowed(#recurse_name)
                } else {
                    std::borrow::Cow::Owned(format!("{}_{}", prefix, #recurse_name))
                };

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
            fn load_environment_with_prefix(&mut self, prefix: std::borrow::Cow<'_, str>) -> anyhow::Result<()> {
                #(#fields)*;
                Ok(())
            }

            fn list_environment_variables_with_prefix(prefix: std::borrow::Cow<'_, str>) -> Vec<crate::cfg::env_overridable::Variable> {
                let mut variables = vec![];
                #(#lists)*;
                variables
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}
