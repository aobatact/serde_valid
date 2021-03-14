use proc_macro_error::abort;
use syn::spanned::Spanned;
use syn::{GenericArgument, Path, PathArguments, PathSegment};

pub fn extract_type_from_option(ty: &syn::Type) -> Option<syn::Type> {
    match *ty {
        syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
        _ => None,
    }
    .and_then(|path| extract_option_segment(path))
    .and_then(|path_segment| {
        let type_params = &path_segment.arguments;
        // It should have only on angle-bracketed param ("<String>"):
        match *type_params {
            PathArguments::AngleBracketed(ref params) => params.args.first(),
            _ => abort!(
                ty.span(),
                "`Option` must be angle bracketed (=`Option<*>`)."
            ),
        }
    })
    .and_then(|generic_arg| match *generic_arg {
        GenericArgument::Type(ref ty) => Some(ty.to_owned()),
        _ => None,
    })
}

fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
    let idents_of_path = path
        .segments
        .iter()
        .into_iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<String>>()
        .join("::");
    if [
        "Option",
        "option::Option",
        "std::option::Option",
        "core::option::Option",
    ]
    .contains(&idents_of_path.as_str())
    {
        path.segments.last()
    } else {
        None
    }
}
