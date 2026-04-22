use syn::Type;

pub(crate) fn ungroup(mut ty: &Type) -> &Type {
    while let Type::Group(group) = ty {
        ty = &group.elem;
    }
    ty
}

/// Determine if the given type is an option and, if so, extract its inner type name
pub(crate) fn as_ty_option(ty: &Type) -> Option<&Type> {
    let Type::Path(ty) = ungroup(ty) else {
        return None;
    };
    let seg = ty.path.segments.last()?;
    let syn::PathArguments::AngleBracketed(bracketed) = &seg.arguments else {
        return None;
    };
    if seg.ident != "Option" || bracketed.args.len() != 1 {
        return None;
    }
    let Some(syn::GenericArgument::Type(arg)) = bracketed.args.get(0) else {
        return None;
    };
    Some(arg)
}

/// Determine whether the final segment of the given type has the given name
pub(crate) fn is_ty_name(name: &str, ty: &Type) -> bool {
    let Type::Path(ty) = ungroup(ty) else {
        return false;
    };
    ty.path
        .segments
        .last()
        .map(|s| s.ident == name)
        .unwrap_or(false)
}
