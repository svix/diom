use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, Expr, Ident, LitInt, Token,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

/// Parsed `#[since(N)]` or `#[since(N, default = EXPR)]` field attribute.
struct SinceAttr {
    version: u32,
    default: Option<Expr>,
}

impl Parse for SinceAttr {
    fn parse(input: ParseStream<'_>) -> Result<Self, syn::Error> {
        let lit: LitInt = input.parse()?;
        let version = lit.base10_parse()?;
        let mut default = None;
        if input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            let key: Ident = input.parse()?;
            if key != "default" {
                return Err(syn::Error::new(key.span(), "expected `default`"));
            }
            let _: Token![=] = input.parse()?;
            default = Some(input.parse()?);
        }
        Ok(SinceAttr { version, default })
    }
}

struct VersionedField {
    ident: Ident,
    ty: syn::Type,
    since: u32,
    default: Option<Expr>,
    nested: bool,
}

fn parse_since(field: &syn::Field) -> Result<Option<SinceAttr>, syn::Error> {
    for attr in &field.attrs {
        if attr.path().is_ident("since") {
            return Ok(Some(attr.parse_args::<SinceAttr>()?));
        }
    }
    Ok(None)
}

/// A `#[nested]` marker stores a struct-valued field with length delimiters (as its own postcard blob) so
/// the nested struct can add trailing fields without corrupting the fields that follow it.
fn parse_nested(field: &syn::Field) -> bool {
    field
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("nested"))
}

/// Parses an optional `#[versioned(row_type = EXPR)]` container attribute. When present, the derive
/// also generates a `TableRow` impl wired to the versioned storage envelope.
fn parse_row_type(input: &DeriveInput) -> Result<Option<Expr>, syn::Error> {
    for attr in &input.attrs {
        if attr.path().is_ident("versioned") {
            let mut row_type = None;
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("row_type") {
                    row_type = Some(meta.value()?.parse::<Expr>()?);
                    Ok(())
                } else {
                    Err(meta.error("expected `row_type`"))
                }
            })?;
            return row_type
                .ok_or_else(|| {
                    syn::Error::new(attr.span(), "missing `row_type` in #[versioned(...)]")
                })
                .map(Some);
        }
    }
    Ok(None)
}

fn collect_fields(input: &DeriveInput) -> Result<Vec<VersionedField>, syn::Error> {
    let syn::Data::Struct(data) = &input.data else {
        return Err(syn::Error::new(
            input.ident.span(),
            "PersistableVersioned can only be derived for structs",
        ));
    };
    let syn::Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new(
            input.ident.span(),
            "PersistableVersioned requires named fields",
        ));
    };
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            input.generics.span(),
            "PersistableVersioned does not support generic types",
        ));
    }

    let mut out = Vec::with_capacity(fields.named.len());
    for field in fields.named.iter() {
        let ident = field.ident.clone().expect("named field");
        let (since, default) = match parse_since(field)? {
            Some(a) => (a.version, a.default),
            None => (0, None),
        };
        out.push(VersionedField {
            ident,
            ty: field.ty.clone(),
            since,
            default,
            nested: parse_nested(field),
        });
    }

    // Fields are read positionally, so a version-`v` reader must be able to stop right after the
    // last field with `since <= v`. That only works if fields are declared in non-decreasing
    // `since` order.
    for w in out.windows(2) {
        if w[1].since < w[0].since {
            return Err(syn::Error::new(
                w[1].ident.span(),
                format!(
                    "field `{}` (since {}) must not precede a field with a higher `since` ({}); \
                     declare fields in non-decreasing `since` order",
                    w[1].ident, w[1].since, w[0].since
                ),
            ));
        }
    }

    Ok(out)
}

pub(crate) fn derive(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let input: DeriveInput = syn::parse2(input)?;
    let fields = collect_fields(&input)?;
    let row_type = parse_row_type(&input)?;
    let name = &input.ident;
    let field_count = fields.len();
    let tuple_len = field_count + 1;
    let write_version = fields.iter().map(|f| f.since).max().unwrap_or(0);

    // When `#[versioned(row_type = ...)]` is present, generate the TableRow impl so the row uses the
    // versioned envelope (its own leading version tag) instead of V0Wrapper. Types without the
    // attribute get only the serde impls, for use as nested (non-row) values.
    let table_row_impl = row_type.map(|row_type| {
        quote! {
            #[automatically_derived]
            impl ::fjall_utils::TableRow for #name {
                const ROW_TYPE: u8 = (#row_type) as u8;
                const VERSIONED: bool = true;
            }
        }
    });

    let serialize_elems = fields.iter().map(|f| {
        let ident = &f.ident;
        if f.nested {
            // Encode the nested value to its own postcard blob and write it with it's own length delimiter, so
            // any trailing fields added to the nested struct later stay contained instead of bleeding into the next
            // field of this struct.
            quote! {
                {
                    let __blob = diom_core::__reexport::postcard::to_allocvec(&self.#ident)
                        .map_err(<__S::Error as ::serde::ser::Error>::custom)?;
                    __tup.serialize_element(&__blob)?;
                }
            }
        } else {
            quote! { __tup.serialize_element(&self.#ident)?; }
        }
    });

    let deserialize_bindings = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        let missing = format!("missing field `{ident}`");
        // How to read the field once we know an element is present. A `#[nested]` field is a
        // length-delimited blob that we decode on its own
        let read_expr = if f.nested {
            quote! {
                {
                    let __blob: ::std::vec::Vec<u8> = __seq
                        .next_element()?
                        .ok_or_else(|| <A::Error as ::serde::de::Error>::custom(#missing))?;
                    diom_core::__reexport::postcard::from_bytes::<#ty>(&__blob)
                        .map_err(<A::Error as ::serde::de::Error>::custom)?
                }
            }
        } else {
            quote! {
                __seq
                    .next_element()?
                    .ok_or_else(|| <A::Error as ::serde::de::Error>::custom(#missing))?
            }
        };
        if f.since == 0 {
            quote! { let #ident: #ty = #read_expr; }
        } else {
            let since = f.since;
            let default_expr = match &f.default {
                Some(expr) => quote! { #expr },
                None => quote! { ::core::default::Default::default() },
            };
            quote! {
                let #ident: #ty = if __version >= #since {
                    #read_expr
                } else {
                    #default_expr
                };
            }
        }
    });

    let field_idents = fields.iter().map(|f| &f.ident);
    let inner_types = fields.iter().map(|f| &f.ty);
    let expecting = format!("{name} versioned tuple");

    // Register this type's shape so the schema detection and checks in CI
    // will work.
    let type_name_str = name.to_string();
    let member_shapes = fields.iter().map(|f| {
        let member_name = f.ident.to_string();
        let ty = &f.ty;
        let ty_str = quote!(#ty).to_string();
        let since = f.since;
        let nested = f.nested;
        quote! {
            diom_core::schema_shape::MemberShape {
                name: #member_name,
                ty: #ty_str,
                since: #since,
                nested: #nested,
            }
        }
    });
    let schema_submit = quote! {
        diom_core::__reexport::inventory::submit! {
            diom_core::schema_shape::SchemaShape {
                module_path: ::core::module_path!(),
                type_name: #type_name_str,
                kind: "versioned",
                members: &[ #(#member_shapes),* ],
            }
        }
    };

    Ok(quote! {
        #[automatically_derived]
        impl ::serde::Serialize for #name {
            fn serialize<__S: ::serde::Serializer>(
                &self,
                __serializer: __S,
            ) -> ::core::result::Result<__S::Ok, __S::Error> {
                use ::serde::ser::SerializeTuple as _;
                let mut __tup = __serializer.serialize_tuple(#tuple_len)?;
                __tup.serialize_element(&(#write_version as u32))?;
                #(#serialize_elems)*
                __tup.end()
            }
        }

        #[automatically_derived]
        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<__D: ::serde::Deserializer<'de>>(
                __deserializer: __D,
            ) -> ::core::result::Result<Self, __D::Error> {
                struct __Visitor;
                impl<'de> ::serde::de::Visitor<'de> for __Visitor {
                    type Value = #name;

                    fn expecting(
                        &self,
                        __f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        __f.write_str(#expecting)
                    }

                    fn visit_seq<A: ::serde::de::SeqAccess<'de>>(
                        self,
                        mut __seq: A,
                    ) -> ::core::result::Result<Self::Value, A::Error> {
                        let __version: u32 = __seq
                            .next_element()?
                            .ok_or_else(|| {
                                <A::Error as ::serde::de::Error>::custom("missing schema version")
                            })?;
                        #(#deserialize_bindings)*
                        ::core::result::Result::Ok(#name { #(#field_idents),* })
                    }
                }
                __deserializer.deserialize_tuple(#tuple_len, __Visitor)
            }
        }

        #[automatically_derived]
        impl #name {
            /// Schema version this build writes as the leading tag on new records.
            ///
            /// Reads accept any version from 0 up to this value. Bumped automatically by adding a
            /// field with a higher `#[since(n)]`.
            pub const WRITE_VERSION: u32 = #write_version;
        }

        #[automatically_derived]
        impl diom_core::persistable_value::PersistableStruct for #name {
            type INNER = ( #(#inner_types,)* );
        }

        #table_row_impl

        #schema_submit
    })
}
