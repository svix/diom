use proc_macro2::TokenStream;
use quote::{ToTokens as _, format_ident, quote};
use syn::{Token, punctuated::Punctuated};

pub(crate) type AideAnnotateArgumentList = Punctuated<syn::MetaNameValue, Token![,]>;

pub(crate) fn expand_aide_annotate(
    args: AideAnnotateArgumentList,
    item: syn::ItemFn,
) -> syn::Result<TokenStream> {
    // By default, use the function's name as the operation id.
    let mut operation_id = item.sig.ident.to_string();
    // The operation summary is the title-cased version of the original
    // function name.
    let mut operation_summary = operation_id
        .split('_')
        .map(title_case)
        .collect::<Vec<String>>()
        .join(" ");
    // Is this op deprecated
    let mut operation_deprecated = false;
    let mut operation_security = true;
    // Whether this operation can return a 413 "Content Too Large" error
    let mut operation_413 = false;
    // The documentation function's name will always be the name of the
    // original function suffixed with `_operation`.
    let operation_ident = format_ident!("{}_operation", item.sig.ident);
    let visibility = item.vis.clone();

    // Allow overriding operation ID and summary via arguments
    for arg in args {
        let arg_as_litbool = || {
            expr_to_litbool(&arg.value).ok_or_else(|| {
                syn::Error::new_spanned(
                    &arg.value,
                    "Unexpected expression, expected a boolean literal",
                )
            })
        };

        let arg_as_litstr = || {
            expr_to_litstr(&arg.value).ok_or_else(|| {
                syn::Error::new_spanned(
                    &arg.value,
                    "Unexpected expression, expected a string literal",
                )
            })
        };

        if arg.path.is_ident("op_id") {
            operation_id = arg_as_litstr()?.value();
        } else if arg.path.is_ident("op_summary") {
            operation_summary = arg_as_litstr()?.value();
        } else if arg.path.is_ident("op_deprecated") {
            operation_deprecated = arg_as_litbool()?.value();
        } else if arg.path.is_ident("op_auth") {
            operation_security = arg_as_litbool()?.value();
        } else if arg.path.is_ident("op_413") {
            operation_413 = arg_as_litbool()?.value();
        } else {
            let path = arg.path.to_token_stream().to_string();
            let msg = format!(
                "Unknown argument `{path}`, expected `op_id` | `op_summary` | `op_deprecated` | \
                 `op_auth` | `op_413`"
            );
            return Err(syn::Error::new_spanned(arg.path, msg));
        }
    }

    let description = doc_comment_from_attributes(&item.attrs);

    if description.is_none() {
        let msg = "An annotated handler must have a doc comment for its description.";
        return Err(syn::Error::new_spanned(&item.sig.ident, msg));
    }

    let add_413_response = operation_413.then(|| {
        quote! {
            .response_with::<413, ::axum::Json<svix_server_core::error::StandardHttpError>, _>(|op| {
                op.description("Payload too large")
            })
        }
    });

    let add_security = operation_security.then(|| {
        quote! {
            .security_requirement("HTTPBearer")
        }
    });

    let set_deprecated = operation_deprecated.then(|| {
        quote! {
            #[cfg(feature = "openapi")]
            { op.inner_mut().deprecated = true; }
        }
    });

    Ok(quote! {
        #item

        #visibility fn #operation_ident(
            op: ::aide::transform::TransformOperation,
        ) -> ::aide::transform::TransformOperation {
            #[cfg(feature = "openapi")]
            #[allow(unused_mut)]
            let mut op = op
                .id(#operation_id)
                .summary(#operation_summary)
                .description(#description)
                #add_413_response
                #add_security
                ;

            #set_deprecated

            op
        }
    })
}

fn doc_comment_from_attributes(attributes: &Vec<syn::Attribute>) -> Option<String> {
    let mut doc_comment_lines = Vec::new();

    for attr in attributes {
        if !attr.path().is_ident("doc") {
            continue;
        }

        // Ignore bare `#[doc]` and `#[doc(foo)]` attributes, only look at `#[doc = "foo"]`
        let Ok(name_val) = attr.meta.require_name_value() else {
            continue;
        };

        // Malformed doc attribute, likely a compile error anyways
        let Some(doc) = expr_to_litstr(&name_val.value) else {
            continue;
        };

        doc_comment_lines.push(doc.value().trim().to_owned());
    }

    if doc_comment_lines.is_empty() {
        return None;
    }
    Some(doc_comment_lines.join("\n"))
}

fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn expr_to_litbool(expr: &syn::Expr) -> Option<&syn::LitBool> {
    match expr {
        syn::Expr::Lit(l) => match &l.lit {
            syn::Lit::Bool(s) => Some(s),
            _ => None,
        },
        _ => None,
    }
}

fn expr_to_litstr(expr: &syn::Expr) -> Option<&syn::LitStr> {
    match expr {
        syn::Expr::Lit(l) => match &l.lit {
            syn::Lit::Str(s) => Some(s),
            _ => None,
        },
        _ => None,
    }
}
