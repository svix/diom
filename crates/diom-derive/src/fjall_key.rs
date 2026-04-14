use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr, Field, Ident, LitInt, spanned::Spanned};

struct KeyField {
    ident: Ident,
    ty: syn::Type,
    index: usize,
}

fn parse_key_index(field: &Field) -> Result<Option<usize>, syn::Error> {
    for attr in &field.attrs {
        if attr.path().is_ident("key") {
            let lit: LitInt = attr.parse_args()?;
            return Ok(Some(lit.base10_parse()?));
        }
    }
    Ok(None)
}

fn parse_prefix(input: &DeriveInput) -> Result<Expr, syn::Error> {
    for attr in &input.attrs {
        if attr.path().is_ident("table_key") {
            let mut prefix = None;
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("prefix") {
                    let value = meta.value()?;
                    prefix = Some(value.parse::<Expr>()?);
                    Ok(())
                } else {
                    Err(meta.error("expected `prefix`"))
                }
            })?;
            return prefix.ok_or_else(|| {
                syn::Error::new(attr.span(), "missing `prefix` in #[table_key(...)]")
            });
        }
    }
    Err(syn::Error::new(
        input.ident.span(),
        "missing #[table_key(prefix = ...)] attribute",
    ))
}

fn collect_fields(input: &DeriveInput) -> Result<Vec<KeyField>, syn::Error> {
    let syn::Data::Struct(data) = &input.data else {
        return Err(syn::Error::new(
            input.ident.span(),
            "FjallKeyAble can only be derived for structs",
        ));
    };

    let syn::Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new(
            input.ident.span(),
            "FjallKeyAble requires named fields",
        ));
    };

    let mut key_fields: Vec<KeyField> = Vec::with_capacity(fields.named.len());

    for field in fields.named.iter() {
        let ident = field.ident.clone().unwrap();
        let ty = field.ty.clone();
        let index = parse_key_index(field)?
            .ok_or_else(|| syn::Error::new(ident.span(), "missing required #[key(N)] attribute"))?;
        key_fields.push(KeyField { ident, ty, index });
    }

    key_fields.sort_by_key(|f| f.index);

    // Validate no duplicate indices
    for w in key_fields.windows(2) {
        if w[0].index == w[1].index {
            return Err(syn::Error::new(
                w[1].ident.span(),
                format!("duplicate key index {}", w[1].index),
            ));
        }
    }

    Ok(key_fields)
}

/// Generates the body of `fjall_key` that builds a key from field references.
///
/// `field_expr` maps each `KeyField` to the token stream used to reference it
/// (e.g. `self.field` or a local binding).
fn gen_key_build(
    prefix: &Expr,
    fields: &[KeyField],
    field_expr: impl Fn(&KeyField) -> TokenStream,
) -> TokenStream {
    let field_refs: Vec<TokenStream> = fields.iter().map(&field_expr).collect();

    quote! {
        let total_len = 1 #(+ ::fjall_utils::KeyComponent::key_len(&#field_refs))*;

        // Use a stack buffer for small keys to avoid heap allocation.
        // Most keys are well under 64 bytes (prefix + a few fixed-size fields).
        let mut stack_buf = [0u8; 64];
        let mut heap_buf;
        let buf: &mut [u8] = if total_len <= stack_buf.len() {
            &mut stack_buf[..total_len]
        } else {
            heap_buf = vec![0u8; total_len];
            &mut heap_buf
        };
        let mut pos = 0;

        buf[pos] = #prefix as u8;
        pos += 1;

        #({
            let written = ::fjall_utils::KeyComponent::write_to_key(&#field_refs, &mut buf[pos..]);
            pos += written;
        })*

        debug_assert_eq!(pos, total_len);
        ::fjall_utils::UserKey::from(&*buf)
    }
}

/// Generates the body of `from_fjall_key` that parses a key back into a struct.
fn gen_key_parse(struct_name: &Ident, prefix: &Expr, fields: &[KeyField]) -> TokenStream {
    // Build parsing statements: one per field in key-index order.
    // Each binds a local `__field_<ident>` so we can construct the struct
    // with the original field names regardless of key ordering.
    let field_parses: Vec<TokenStream> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let local = quote::format_ident!("__field_{}", ident);
            let field_name = ident.to_string();
            quote! {
                let (#local, consumed) =
                    ::fjall_utils::KeyComponent::read_from_key(&bytes[pos..])
                        .map_err(|e| ::std::borrow::Cow::Owned(::std::format!(
                            "failed to parse field `{}`: {}",
                            #field_name, e
                        )))?;
                pos += consumed;
            }
        })
        .collect();

    let field_constructs: Vec<TokenStream> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let local = quote::format_ident!("__field_{}", ident);
            quote! { #ident: #local }
        })
        .collect();

    quote! {
        let bytes: &[u8] = &key;
        if bytes.first().copied() != ::std::option::Option::Some(#prefix as u8) {
            return ::std::result::Result::Err(::std::borrow::Cow::Owned(::std::format!(
                "key does not start with expected prefix {} (for {})",
                #prefix as u8, ::std::stringify!(#struct_name)
            )));
        }
        let mut pos = 1;

        #(#field_parses)*

        if pos != bytes.len() {
            return ::std::result::Result::Err(::std::borrow::Cow::Owned(::std::format!(
                "trailing bytes in key: consumed {} of {}",
                pos, bytes.len()
            )));
        }

        ::std::result::Result::Ok(Self { #(#field_constructs),* })
    }
}

/// Generates `build_key()`: takes all fields by reference, returns `UserKey`.
fn gen_build_key_method(prefix: &Expr, fields: &[KeyField]) -> TokenStream {
    let params: Vec<TokenStream> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            quote! { #ident: &#ty }
        })
        .collect();

    let body = gen_key_build(prefix, fields, |f| {
        let ident = &f.ident;
        quote! { *#ident }
    });

    quote! {
        /// Serialize a key from borrowed fields without constructing the struct.
        pub fn build_key(#(#params),*) -> ::fjall_utils::UserKey {
            #body
        }
    }
}

/// Generates a `prefix_<last_field>()` static method that builds a key prefix
/// from the first N fields.
fn gen_prefix_method(_prefix: &Expr, fields: &[KeyField]) -> TokenStream {
    let last_field = &fields[fields.len() - 1];
    let method_name = quote::format_ident!("prefix_{}", last_field.ident);

    let params: Vec<TokenStream> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            quote! { #ident: &#ty }
        })
        .collect();

    let types: Vec<&syn::Type> = fields.iter().map(|f| &f.ty).collect();
    let len_expr = quote! {
        1usize #(+ <#types as ::fjall_utils::KeyComponent>::BYTE_SIZE)*
    };

    let writes: Vec<TokenStream> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            quote! {
                pos += ::fjall_utils::KeyComponent::write_to_key(#ident, &mut buf[pos..]);
            }
        })
        .collect();

    let field_names: Vec<String> = fields.iter().map(|f| f.ident.to_string()).collect();
    let doc = format!(
        "Returns the key prefix: [PREFIX][{}].",
        field_names.join("][")
    );

    quote! {
        #[doc = #doc]
        pub fn #method_name(#(#params),*) -> ::std::vec::Vec<u8> {
            let total_len = #len_expr;
            let mut buf = ::std::vec![0u8; total_len];
            let mut pos = 0;

            buf[pos] = <Self as ::fjall_utils::FjallKeyAble>::PREFIX;
            pos += 1;

            #(#writes)*

            ::std::debug_assert_eq!(pos, total_len);
            buf
        }
    }
}

pub(crate) fn derive(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let input: DeriveInput = syn::parse2(input)?;
    let prefix = parse_prefix(&input)?;
    let fields = collect_fields(&input)?;
    let name = &input.ident;

    // All fields except the last (by key order) must be FIXED_SIZE.
    let fixed_size_assertions: Vec<TokenStream> = fields[..fields.len().saturating_sub(1)]
        .iter()
        .map(|kf| {
            let ident = &kf.ident;
            let ty = &kf.ty;
            let msg = format!(
                "field `{ident}` must be fixed-size (only the last key field can be variable-size)"
            );
            quote! {
                const _: () = ::std::assert!(
                    <#ty as ::fjall_utils::KeyComponent>::FIXED_SIZE,
                    #msg
                );
            }
        })
        .collect();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fjall_key_body = gen_key_build(&prefix, &fields, |f| {
        let ident = &f.ident;
        quote! { self.#ident }
    });

    let from_fjall_key_body = gen_key_parse(name, &prefix, &fields);

    // Generate extract_<field> methods for each field.
    // Each method reads a single field from a raw key without constructing the
    // full struct. It skips preceding fixed-size fields using BYTE_SIZE, then
    // reads the target field via read_ref_from_key to avoid allocation.
    let extract_methods: Vec<TokenStream> = fields
        .iter()
        .enumerate()
        .map(|(pos, kf)| {
            let ident = &kf.ident;
            let ty = &kf.ty;
            let method_name = quote::format_ident!("extract_{}", ident);
            let doc = format!(
                "Extracts the `{ident}` field from a raw key without \
                 constructing the full struct."
            );

            // Sum BYTE_SIZE of all preceding fields (all fixed-size, enforced by assertion)
            let preceding_types: Vec<&syn::Type> = fields[..pos].iter().map(|f| &f.ty).collect();
            let skip = if preceding_types.is_empty() {
                quote! { 1usize }
            } else {
                quote! { 1usize #(+ <#preceding_types as ::fjall_utils::KeyComponent>::BYTE_SIZE)* }
            };

            quote! {
                #[doc = #doc]
                pub fn #method_name(key: &::fjall_utils::UserKey) -> ::std::result::Result<
                    <#ty as ::fjall_utils::KeyComponent>::Ref<'_>,
                    ::std::borrow::Cow<'static, str>,
                > {
                    let bytes: &[u8] = key;
                    if bytes.first().copied() != ::std::option::Option::Some(#prefix as u8) {
                        return ::std::result::Result::Err(::std::borrow::Cow::Owned(
                            ::std::format!(
                                "key does not start with expected prefix {} (for {})",
                                #prefix as u8, ::std::stringify!(#name)
                            ),
                        ));
                    }
                    let pos = #skip;
                    let (val, _) = <#ty as ::fjall_utils::KeyComponent>::read_ref_from_key(
                        &bytes[pos..],
                    )?;
                    ::std::result::Result::Ok(val)
                }
            }
        })
        .collect();

    // Generate prefix_<field> methods for each leading subsequence of fixed-size fields.
    // For position i in 0..fields.len()-1, generate a method that builds
    // [PREFIX][field_0]...[field_i]. Skip structs with only 1 field.
    let prefix_methods: Vec<TokenStream> = if fields.len() >= 2 {
        (0..fields.len() - 1)
            .map(|i| gen_prefix_method(&prefix, &fields[..=i]))
            .collect()
    } else {
        Vec::new()
    };

    let build_key_method = gen_build_key_method(&prefix, &fields);

    Ok(quote! {
        #(#fixed_size_assertions)*

        impl #impl_generics ::fjall_utils::FjallKeyAble for #name #ty_generics #where_clause {
            const PREFIX: u8 = #prefix as u8;

            fn fjall_key(&self) -> ::fjall_utils::UserKey {
                #fjall_key_body
            }

            fn from_fjall_key(
                key: ::fjall_utils::UserKey,
            ) -> ::std::result::Result<Self, ::std::borrow::Cow<'static, str>> {
                #from_fjall_key_body
            }
        }

        #[allow(dead_code)]
        impl #impl_generics #name #ty_generics #where_clause {
            #(#extract_methods)*
            #(#prefix_methods)*
            #build_key_method
        }
    })
}
