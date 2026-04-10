use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, Ident, LitInt, spanned::Spanned};

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

fn parse_prefix(input: &DeriveInput) -> Result<LitInt, syn::Error> {
    for attr in &input.attrs {
        if attr.path().is_ident("table_key") {
            let mut prefix = None;
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("prefix") {
                    let value = meta.value()?;
                    let lit = value.parse::<LitInt>()?;
                    // Validate it fits in a u8
                    lit.base10_parse::<u8>().map_err(|e| {
                        syn::Error::new(lit.span(), format!("prefix must be a u8 (0-255): {e}"))
                    })?;
                    prefix = Some(lit);
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
        "missing #[table_key(prefix = <u8>)] attribute",
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
    prefix: &LitInt,
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
        let buf: &mut [u8] = if total_len <= 64 {
            &mut stack_buf[..total_len]
        } else {
            heap_buf = vec![0u8; total_len];
            &mut heap_buf
        };
        let mut pos = 0;

        buf[pos] = #prefix;
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
fn gen_key_parse(struct_name: &Ident, prefix: &LitInt, fields: &[KeyField]) -> TokenStream {
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
        if bytes.first().copied() != ::std::option::Option::Some(#prefix) {
            return ::std::result::Result::Err(::std::borrow::Cow::Owned(::std::format!(
                "key does not start with expected prefix {} (for {})",
                #prefix, ::std::stringify!(#struct_name)
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

pub(crate) fn derive(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let input: DeriveInput = syn::parse2(input)?;
    let prefix = parse_prefix(&input)?;
    let prefix_val: u8 = prefix.base10_parse()?;
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
    let uniqueness_symbol = quote::format_ident!("__FJALL_KEY_PREFIX_{}", prefix_val);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fjall_key_body = gen_key_build(&prefix, &fields, |f| {
        let ident = &f.ident;
        quote! { self.#ident }
    });

    let from_fjall_key_body = gen_key_parse(name, &prefix, &fields);

    let range_build_start = gen_key_build(&prefix, &fields, |f| {
        let ident = &f.ident;
        quote! { start.#ident }
    });

    let range_build_end = gen_key_build(&prefix, &fields, |f| {
        let ident = &f.ident;
        quote! { end.#ident }
    });

    let range_build_end_only = gen_key_build(&prefix, &fields, |f| {
        let ident = &f.ident;
        quote! { end.#ident }
    });

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
                fn #method_name(key: &::fjall_utils::UserKey) -> ::std::result::Result<
                    <#ty as ::fjall_utils::KeyComponent>::Ref<'_>,
                    ::std::borrow::Cow<'static, str>,
                > {
                    let bytes: &[u8] = key;
                    if bytes.first().copied() != ::std::option::Option::Some(#prefix) {
                        return ::std::result::Result::Err(::std::borrow::Cow::Owned(
                            ::std::format!(
                                "key does not start with expected prefix {} (for {})",
                                #prefix, ::std::stringify!(#name)
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

    Ok(quote! {
        /// Emits a linker symbol named after the prefix value (e.g. `__FJALL_KEY_PREFIX_1`).
        /// If two types claim the same prefix, the duplicate symbol causes a compile-time
        /// error, enforcing global uniqueness across the binary.
        ///
        /// SAFETY: the symbol is a zero-sized `()` value with no consumers — `no_mangle`
        /// is used purely to produce a deterministic name for collision detection.
        #[unsafe(no_mangle)]
        #[used]
        static #uniqueness_symbol: () = ();

        #(#fixed_size_assertions)*

        impl #impl_generics ::fjall_utils::FjallKeyAble for #name #ty_generics #where_clause {
            fn fjall_key(&self) -> ::fjall_utils::UserKey {
                #fjall_key_body
            }

            fn from_fjall_key(
                key: ::fjall_utils::UserKey,
            ) -> ::std::result::Result<Self, ::std::borrow::Cow<'static, str>> {
                #from_fjall_key_body
            }

            fn range(start: Self, end: Self) -> ::std::ops::Range<::fjall_utils::UserKey> {
                let start_key = { #range_build_start };
                let end_key = { #range_build_end };
                start_key..end_key
            }

            fn range_inclusive(start: Self, end: Self) -> ::std::ops::RangeInclusive<::fjall_utils::UserKey> {
                let start_key = { #range_build_start };
                let end_key = { #range_build_end };
                start_key..=end_key
            }

            fn range_end_inclusive(end: Self) -> ::std::ops::RangeToInclusive<::fjall_utils::UserKey> {
                let end_key = { #range_build_end_only };
                ..=end_key
            }
        }

        #[allow(dead_code)]
        impl #impl_generics #name #ty_generics #where_clause {
            #(#extract_methods)*
        }
    })
}
