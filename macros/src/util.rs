pub fn is_option(ty: &syn::Type) -> bool {
    if let syn::Type::Path(ref p) = ty {
        let leading_path = &p
            .path
            .segments
            .iter()
            .take(3)
            .into_iter()
            .fold("".to_owned(), |acc, el| acc + "::" + &el.ident.to_string())[2..];

        return match leading_path {
            "Option" | "std::option::Option" | "core::option::Option" => true,
            _ => false,
        };
    };
    false
}

pub fn unwrap_option(ty: &syn::Type) -> Option<&syn::Type> {
    use syn::{GenericArgument, Path, PathArguments, PathSegment, Type};

    fn extract_type_path(ty: &Type) -> Option<&Path> {
        match *ty {
            Type::Path(ref type_path) if type_path.qself.is_none() => Some(&type_path.path),
            _ => None,
        }
    }

    fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
        let idents = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |acc, el| acc + &el.ident.to_string() + "::");

        [
            "Option::",
            "std::option::Option::",
            "core::option::Option::",
        ]
        .into_iter()
        .find(|s| &idents == *s)
        .and_then(|_| path.segments.last())
    }

    extract_type_path(ty)
        .and_then(|path| extract_option_segment(path))
        .and_then(|segment| {
            let type_params = &segment.arguments;

            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}
