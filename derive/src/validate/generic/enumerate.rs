use crate::types::Field;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

type Lits<'a> = syn::punctuated::Punctuated<&'a syn::Lit, syn::token::Comma>;

pub fn extract_generic_enumerate_validator(
    field: &impl Field,
    attribute: &syn::Attribute,
    item_list: &syn::MetaList,
    message_fn: Option<TokenStream>,
) -> Result<Validator, crate::Errors> {
    if let Some(array_field) = field.array_field() {
        Ok(Validator::Array(Box::new(
            extract_generic_enumerate_validator(&array_field, attribute, item_list, message_fn)?,
        )))
    } else if let Some(option_field) = field.option_field() {
        Ok(Validator::Option(Box::new(
            extract_generic_enumerate_validator(&option_field, attribute, item_list, message_fn)?,
        )))
    } else {
        Ok(Validator::Normal(
            inner_extract_generic_enumerate_validator(field, item_list, message_fn)?,
        ))
    }
}

fn inner_extract_generic_enumerate_validator(
    field: &impl Field,
    item_list: &syn::MetaList,
    message_fn: Option<TokenStream>,
) -> Result<TokenStream, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();

    let enumerate = get_enumerate(item_list)?;
    let message = message_fn.unwrap_or(quote!(
        ::serde_valid::EnumerateErrorParams::to_default_message
    ));

    Ok(quote!(
        if !::serde_valid::validate_generic_enumerate(
            #field_ident,
            &[#enumerate],
        ) {
            use ::serde_valid::error::ToDefaultMessage;
            __errors
                .entry(#field_name)
                .or_default()
                .push(::serde_valid::validation::Error::Enumerate(
                    ::serde_valid::error::Message::new(
                        ::serde_valid::EnumerateErrorParams::new(
                            #field_ident,
                            &[#enumerate],
                        ),
                        #message
                )
                ));
        }
    ))
}

fn get_enumerate<'a>(
    syn::MetaList { path, nested, .. }: &'a syn::MetaList,
) -> Result<Lits<'a>, crate::Errors> {
    let mut errors = vec![];
    let mut enumerate = Lits::new();

    if nested.len() == 0 {
        errors.push(crate::Error::validate_enumerate_need_item(path));
    }
    for item in nested {
        match item {
            syn::NestedMeta::Lit(lit) => enumerate.push(lit),
            syn::NestedMeta::Meta(meta) => errors.push(crate::Error::literal_only(meta)),
        }
    }

    if errors.is_empty() {
        Ok(enumerate)
    } else {
        Err(errors)
    }
}
