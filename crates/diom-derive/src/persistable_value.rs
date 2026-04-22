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

fn parse_struct(
    obj: &DataStruct,
    input: &DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut inner = Vec::with_capacity(obj.fields.len());

    check_serde_container_attrs(&input.attrs)?;

    for field in &obj.fields {
        check_serde_field_attrs(field)?;

        let ty = &field.ty;
        inner.push(quote! { #ty });
    }

    let ident = &input.ident;

    Ok(quote! {
        #[allow(unsafe_code)]
        #[automatically_derived]
        impl #impl_generics diom_core::persistable_value::PersistableStruct for #ident #ty_generics #where_clause {
            type INNER = ( #(#inner,)* );
        }
    })
}

fn parse_enum(obj: &DataEnum, input: &DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
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

    Ok(quote! {
        #[allow(unsafe_code)]
        #[automatically_derived]
        impl #impl_generics diom_core::persistable_value::PersistableStruct for #ident #ty_generics #where_clause {
            type INNER = ( #(#inner,)* );
        }
    })
}

pub(crate) fn derive_persistable_value(input: DeriveInput) -> proc_macro2::TokenStream {
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
