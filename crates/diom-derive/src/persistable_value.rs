use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DataEnum, DataStruct, DeriveInput, Field, LitStr, Token};

fn check_serde_container_attrs(attrs: &[Attribute]) -> Result<(), syn::Error> {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            for ident in ["untagged", "tag", "default"] {
                if meta.path.is_ident(ident) {
                    return Err(
                        meta.error(format!("{ident} is unsafe on persistable value containers"))
                    );
                }
            }
            // chomp any argument
            if meta.input.peek(Token![=]) {
                meta.value()?;
                let _: LitStr = meta.input.parse()?;
            }
            Ok(())
        })?;
    }
    Ok(())
}

fn check_serde_field_attrs(field: &Field) -> Result<(), syn::Error> {
    for attr in &field.attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            for ident in ["skip_serializing_if", "alias", "flatten"] {
                if meta.path.is_ident(ident) {
                    return Err(
                        meta.error(format!("{ident} is unsafe on persistable value fields"))
                    );
                }
            }
            // chomp any argument
            if meta.input.peek(Token![=]) {
                meta.value()?;
                let _: LitStr = meta.input.parse()?;
            }
            Ok(())
        })?;
    }
    Ok(())
}

/// Builds the `inventory::submit!` that registers this type's shape for the schema-manifest guard.
/// Generic types are skipped, since one registration cannot describe every instantiation.
fn schema_submit(input: &DeriveInput, kind: &str, members: Vec<TokenStream>) -> TokenStream {
    if !input.generics.params.is_empty() {
        return quote! {};
    }
    let type_name = input.ident.to_string();
    quote! {
        diom_core::__reexport::inventory::submit! {
            diom_core::schema_shape::SchemaShape {
                module_path: ::core::module_path!(),
                type_name: #type_name,
                kind: #kind,
                members: &[ #(#members),* ],
            }
        }
    }
}

fn struct_members(obj: &DataStruct) -> Vec<TokenStream> {
    obj.fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let name = field
                .ident
                .as_ref()
                .map(|id| id.to_string())
                .unwrap_or_else(|| i.to_string());
            let ty = &field.ty;
            let ty_str = quote!(#ty).to_string();
            quote! {
                diom_core::schema_shape::MemberShape { name: #name, ty: #ty_str, since: 0u32, nested: false }
            }
        })
        .collect()
}

fn enum_members(obj: &DataEnum) -> Vec<TokenStream> {
    obj.variants
        .iter()
        .map(|variant| {
            let name = variant.ident.to_string();
            let ty_str = variant
                .fields
                .iter()
                .map(|f| {
                    let ty = &f.ty;
                    quote!(#ty).to_string()
                })
                .collect::<Vec<_>>()
                .join(", ");
            quote! {
                diom_core::schema_shape::MemberShape { name: #name, ty: #ty_str, since: 0u32, nested: false }
            }
        })
        .collect()
}

fn parse_struct(obj: &DataStruct, input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut inner = Vec::with_capacity(obj.fields.len());

    check_serde_container_attrs(&input.attrs)?;

    for field in &obj.fields {
        check_serde_field_attrs(field)?;

        let ty = &field.ty;
        inner.push(quote! { #ty });
    }

    let ident = &input.ident;

    let submit = schema_submit(input, "value-struct", struct_members(obj));

    Ok(quote! {
        #[allow(unsafe_code)]
        #[automatically_derived]
        impl #impl_generics diom_core::persistable_value::PersistableStruct for #ident #ty_generics #where_clause {
            type INNER = ( #(#inner,)* );
        }

        #submit
    })
}

fn parse_enum(obj: &DataEnum, input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut inner = Vec::with_capacity(obj.variants.len());

    check_serde_container_attrs(&input.attrs)?;

    for variant in obj.variants.iter() {
        for field in &variant.fields {
            let ty = &field.ty;

            check_serde_field_attrs(field)?;

            inner.push(quote! { #ty });
        }
    }

    let ident = &input.ident;

    let submit = schema_submit(input, "value-enum", enum_members(obj));

    Ok(quote! {
        #[allow(unsafe_code)]
        #[automatically_derived]
        impl #impl_generics diom_core::persistable_value::PersistableStruct for #ident #ty_generics #where_clause {
            type INNER = ( #(#inner,)* );
        }

        #submit
    })
}

pub(crate) fn derive_persistable_value(input: DeriveInput) -> TokenStream {
    let expanded = match &input.data {
        syn::Data::Enum(obj) => parse_enum(obj, &input),
        syn::Data::Struct(obj) => parse_struct(obj, &input),
        _ => {
            return quote! { compile_error!("This macro may only be applied to structs and enums") };
        }
    };

    // Hand the output tokens back to the compiler.
    match expanded {
        Ok(expanded) => expanded,
        Err(e) => e.to_compile_error(),
    }
}
