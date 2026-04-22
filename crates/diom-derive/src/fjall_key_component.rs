use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn derive(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let input: DeriveInput = syn::parse2(input)?;
    let name = &input.ident;

    let syn::Data::Struct(data) = &input.data else {
        return Err(syn::Error::new(
            input.ident.span(),
            "FjallKeyComponent can only be derived for structs",
        ));
    };

    let syn::Fields::Unnamed(fields) = &data.fields else {
        return Err(syn::Error::new(
            input.ident.span(),
            "FjallKeyComponent derive requires a tuple struct",
        ));
    };

    if fields.unnamed.len() != 1 {
        return Err(syn::Error::new(
            input.ident.span(),
            "FjallKeyComponent derive requires exactly one field",
        ));
    }

    let inner_ty = &fields.unnamed[0].ty;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::fjall_utils::FjallKeyComponent for #name #ty_generics #where_clause {
            const FIXED_SIZE: bool = <#inner_ty as ::fjall_utils::FjallKeyComponent>::FIXED_SIZE;
            const BYTE_SIZE: usize = <#inner_ty as ::fjall_utils::FjallKeyComponent>::BYTE_SIZE;
            type Ref<'a> = Self;

            fn key_len(&self) -> usize {
                ::fjall_utils::FjallKeyComponent::key_len(&self.0)
            }

            fn write_to_key(&self, buf: &mut [u8]) -> usize {
                ::fjall_utils::FjallKeyComponent::write_to_key(&self.0, buf)
            }

            fn read_from_key(
                buf: &[u8],
            ) -> ::std::result::Result<(Self, usize), ::std::borrow::Cow<'static, str>> {
                let (val, len) = <#inner_ty as ::fjall_utils::FjallKeyComponent>::read_from_key(buf)?;
                ::std::result::Result::Ok((Self(val), len))
            }

            fn read_ref_from_key(
                buf: &[u8],
            ) -> ::std::result::Result<(Self::Ref<'_>, usize), ::std::borrow::Cow<'static, str>> {
                Self::read_from_key(buf)
            }
        }
    })
}
