use quote::quote;
use syn::{DeriveInput, Ident, LitStr, Token};

use crate::utils::as_ty_option;

struct DumpableField {
    name: String,
    field: Ident,
    docstring: Vec<String>,
    nest: bool,
    is_optional: bool,
}

impl DumpableField {
    fn parse(field: &syn::Field) -> Result<Option<Self>, syn::Error> {
        let mut render = true;
        let Some(ident) = &field.ident else {
            return Ok(None);
        };
        let mut nest = false;
        let mut name = ident.to_string();
        let is_optional = as_ty_option(&field.ty).is_some();

        let mut docstring = vec![];

        for attr in &field.attrs {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(nv) = &attr.meta
                    && let syn::Expr::Lit(lit) = &nv.value
                    && let syn::Lit::Str(lit) = &lit.lit
                {
                    let value = lit.value();
                    docstring.push(value.trim_start().to_string())
                }
            } else if attr.path().is_ident("dumpable_config") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        render = false;
                    } else if meta.path.is_ident("nest") {
                        nest = true;
                    } else {
                        return Err(meta.error("unrecognized attr"));
                    }
                    Ok(())
                })?;
            } else if attr.path().is_ident("serde") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        render = false;
                    } else if meta.path.is_ident("rename") {
                        meta.value()?;
                        let rename_to: LitStr = meta.input.parse()?;
                        name = rename_to.value();
                    } else if meta.path.is_ident("default") && meta.input.peek(Token![=]) {
                        meta.value()?;
                        let _: LitStr = meta.input.parse()?;
                    }
                    Ok(())
                })?;
            }
        }
        if !render {
            return Ok(None);
        }

        Ok(Some(Self {
            name,
            field: ident.clone(),
            nest,
            docstring,
            is_optional,
        }))
    }
}

pub(crate) fn derive_dumpable_config(input: DeriveInput) -> proc_macro2::TokenStream {
    let syn::Data::Struct(obj) = input.data else {
        return quote! { compile_error!("This macro may only be applied to structs") };
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut fields = Vec::with_capacity(obj.fields.len());
    for field in obj.fields.iter() {
        let parsed = match DumpableField::parse(field) {
            Ok(Some(field)) => field,
            Ok(None) => continue,
            Err(e) => return e.to_compile_error(),
        };
        fields.push(parsed);
    }

    fields.sort_by_key(|field| (field.nest, field.name.clone()));
    let mut lists = Vec::with_capacity(obj.fields.len());
    for parsed in fields {
        let field = parsed.field;
        let name = parsed.name;
        let is_optional = parsed.is_optional;
        let docs = parsed.docstring.iter().map(|d| {
            let format_string = if d.trim().is_empty() {
                "#{}\n"
            } else {
                "# {}\n"
            };
            quote! { write!(writer, #format_string, #d.trim())?; }
        });

        let lister = if parsed.nest {
            quote! {
                let new_prefix = if prefix.is_empty() {
                    format!("{}", #name)
                } else {
                    format!("{}.{}", prefix, #name)
                };
                self.#field.dump_map(writer, new_prefix)?;
            }
        } else {
            let dump_serialized = quote! {
                let serialized = serde::Serialize::serialize(
                    &self.#field,
                    toml::ser::ValueSerializer::new(&mut buffer)
                )?;
                write!(writer, "{} = {}\n", #name, serialized)?;
                buffer.clear();
            };

            if is_optional {
                quote! {
                    if self.#field.is_none() {
                        write!(writer, "# {} =\n", #name)?;
                    } else {
                        #dump_serialized
                    }
                }
            } else {
                dump_serialized
            }
        };

        lists.push(quote! {
            writeln!(writer)?;
            #(#docs)*;
            #lister
        });
    }

    let name = input.ident;

    quote! {
        impl #impl_generics crate::cfg::dumpable_config::DumpableConfig for #name #ty_generics #where_clause {
            fn dump_fields<W: std::io::Write>(&self, writer: &mut W, prefix: String) -> ::anyhow::Result<()> {
                let mut buffer = String::new();
                #(#lists)*;
                Ok(())
            }
        }
    }
}
