use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, LitStr, Token, parenthesized};

use crate::utils::{as_ty_option, is_ty_name};

struct EoField {
    variable: String,
    field: Ident,
    is_optional: bool,
    is_vec: bool,
    flatten: bool,
    docstring: Vec<String>,
    nest: Option<String>,
    ty: syn::Type,
}

impl EoField {
    fn parse(field: &syn::Field) -> Result<Option<Self>, syn::Error> {
        let mut render = true;
        let mut flatten = false;
        let Some(ident) = &field.ident else {
            return Ok(None);
        };
        let mut nest = None;

        let is_vec;
        let is_duration;
        let is_optional;
        let mut docstring_lines: Vec<String> = vec![];
        let mut current_docstring = String::new();

        if let Some(option_inner) = as_ty_option(&field.ty) {
            is_optional = true;
            is_duration = is_ty_name("DurationMs", option_inner)
                || is_ty_name("NonZeroDurationMs", option_inner);
            is_vec = is_ty_name("Vec", option_inner);
        } else {
            is_optional = false;
            is_duration =
                is_ty_name("DurationMs", &field.ty) || is_ty_name("NonZeroDurationMs", &field.ty);
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
                    if value.trim_start().is_empty() {
                        if !current_docstring.is_empty() {
                            docstring_lines.push(current_docstring.trim().to_owned());
                            current_docstring = String::new();
                        }
                    } else {
                        if !(current_docstring.is_empty()
                            || current_docstring.ends_with(|f: char| f.is_whitespace()))
                        {
                            current_docstring.push(' ')
                        }
                        current_docstring.push_str(value.trim_start())
                    }
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
                        nest = Some(recursive_prefix);
                        Ok(())
                    } else {
                        Err(meta.error("unrecognized repr"))
                    }
                })?;
            } else if attr.path().is_ident("serde") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("flatten") {
                        flatten = true;
                    } else if meta.input.peek(Token![=]) {
                        // chomp any argument
                        meta.value()?;
                        let _: LitStr = meta.input.parse()?;
                    }
                    Ok(())
                })?;
            }
        }
        if !current_docstring.is_empty() {
            docstring_lines.push(current_docstring);
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
            flatten,
            docstring: docstring_lines,
            ty: field.ty.clone(),
        }))
    }
}

pub(crate) fn derive_env_overridable(input: DeriveInput) -> TokenStream {
    let syn::Data::Struct(obj) = input.data else {
        return quote! { compile_error!("This macro may only be applied to structs") };
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut fields = Vec::with_capacity(obj.fields.len());
    let mut lists = Vec::with_capacity(obj.fields.len());
    for field in obj.fields.iter() {
        let parsed = match EoField::parse(field) {
            Ok(Some(field)) => field,
            Ok(None) => continue,
            Err(e) => return e.to_compile_error(),
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
        } else {
            quote! { crate::cfg::env_overridable::env_var }
        };

        let field_parser = if let Some(recurse_name) = &parsed.nest {
            quote! {
                let name = format!("{}_{}", prefix, #recurse_name);
                self.#field.load_environment_with_prefix(name)?;
            }
        } else if parsed.flatten {
            quote! {
                self.#field.load_environment_with_prefix(prefix.clone())?;
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

        let field_ty = &parsed.ty;
        let lister = if let Some(recurse_name) = &parsed.nest {
            quote! {
                let name = format!("{}_{}", prefix, #recurse_name);
                variables.extend(<#field_ty>::list_environment_variables_with_prefix(name));
            }
        } else if parsed.flatten {
            quote! {
                variables.extend(<#field_ty>::list_environment_variables_with_prefix(prefix.clone()));
            }
        } else {
            let docstring = if parsed.docstring.is_empty() {
                quote! { None }
            } else {
                let ds = parsed.docstring.join("\n\n");
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
        #[automatically_derived]
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
    expanded
}
