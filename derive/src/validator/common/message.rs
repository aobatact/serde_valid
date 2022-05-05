use crate::types::SingleIdentPath;
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::spanned::Spanned;

use super::{get_str, MetaListMessage, MetaNameValueMessage, MetaPathMessage};

pub fn extract_message_fn_tokens(
    nested_meta: &syn::NestedMeta,
) -> Result<TokenStream, crate::Error> {
    match nested_meta {
        syn::NestedMeta::Meta(meta) => match meta {
            syn::Meta::List(message_fn_list) => {
                extract_message_fn_tokens_from_meta_list(message_fn_list)
            }
            syn::Meta::NameValue(name_value) => {
                extract_message_fn_tokens_from_name_value(name_value)
            }
            syn::Meta::Path(path) => {
                let path_label = SingleIdentPath::new(path).ident().to_string();
                if MetaNameValueMessage::from_str(&path_label).is_ok() {
                    Err(crate::Error::new_meta_name_value_need_value_error(
                        path.span(),
                        &path_label,
                    ))
                } else if MetaListMessage::from_str(&path_label).is_ok() {
                    Err(crate::Error::new_meta_list_need_value_error(
                        path.span(),
                        &path_label,
                    ))
                } else {
                    Err(crate::Error::new_unknown_meta_error(
                        path.span(),
                        &path_label,
                        &(MetaNameValueMessage::iter().map(|x| x.name()))
                            .chain(MetaListMessage::iter().map(|x| x.name()))
                            .chain(MetaPathMessage::iter().map(|x| x.name()))
                            .collect::<Vec<_>>(),
                    ))
                }
            }
        },
        syn::NestedMeta::Lit(lit) => Err(crate::Error::literal_not_support(lit)),
    }
}

fn extract_message_fn_tokens_from_meta_list(
    syn::MetaList {
        path,
        nested: message_fn_define,
        ..
    }: &syn::MetaList,
) -> Result<TokenStream, crate::Error> {
    let path_ident = SingleIdentPath::new(&path).ident();
    let path_label = path_ident.to_string();

    match MetaListMessage::from_str(&path_label) {
        Ok(MetaListMessage::MessageFn) => get_message_fn_from_nested_meta(message_fn_define),
        Err(unknown) => {
            if MetaNameValueMessage::from_str(&path_label).is_ok() {
                Err(crate::Error::new_meta_list_need_value_error(
                    path.span(),
                    &path_label,
                ))
            } else if MetaPathMessage::from_str(&path_label).is_ok() {
                Err(crate::Error::new_meta_path_need_value_error(
                    path.span(),
                    &path_label,
                ))
            } else {
                Err(crate::Error::new_unknown_meta_error(
                    path_ident.span(),
                    &unknown,
                    &MetaListMessage::iter()
                        .map(|x| x.name())
                        .collect::<Vec<_>>(),
                ))
            }
        }
    }
}

fn extract_message_fn_tokens_from_name_value(
    syn::MetaNameValue { path, lit, .. }: &syn::MetaNameValue,
) -> Result<TokenStream, crate::Error> {
    let path_ident = SingleIdentPath::new(&path).ident();
    let path_label = path_ident.to_string();

    match MetaNameValueMessage::from_str(&path_label) {
        Ok(MetaNameValueMessage::Message) => get_message_fn_from_lit(lit),
        Err(unknown) => {
            if MetaListMessage::from_str(&path_label).is_ok() {
                Err(crate::Error::new_meta_list_need_value_error(
                    path.span(),
                    &path_label,
                ))
            } else if MetaPathMessage::from_str(&path_label).is_ok() {
                Err(crate::Error::new_meta_path_need_value_error(
                    path.span(),
                    &path_label,
                ))
            } else {
                Err(crate::Error::new_unknown_meta_error(
                    path_ident.span(),
                    &unknown,
                    &MetaNameValueMessage::iter()
                        .map(|x| x.name())
                        .collect::<Vec<_>>(),
                ))
            }
        }
    }
}

fn get_message_fn_from_nested_meta(
    message_fn_define: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> Result<TokenStream, crate::Error> {
    match message_fn_define.len() {
        0 => Err(crate::Error::new_message_fn_name_error(
            message_fn_define.span(),
        )),
        1 => match &message_fn_define[0] {
            syn::NestedMeta::Meta(ref meta) => match meta {
                syn::Meta::Path(fn_name) => Ok(quote!(#fn_name)),
                syn::Meta::List(list) => Err(crate::Error::new_message_fn_name_error(list.span())),
                syn::Meta::NameValue(name_value) => {
                    Err(crate::Error::new_message_fn_name_error(name_value.span()))
                }
            },
            syn::NestedMeta::Lit(lit) => Err(crate::Error::new_message_fn_name_error(lit.span())),
        },
        _ => {
            let mut args: syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma> =
                syn::punctuated::Punctuated::new();
            for arg in message_fn_define.iter().skip(1) {
                args.push(arg.clone())
            }
            Err(crate::Error::message_fn_tail_error(args.span()))
        }
    }
}

fn get_message_fn_from_lit(lit: &syn::Lit) -> Result<TokenStream, crate::Error> {
    get_str(lit).map(|lit_str| quote!(|_| { #lit_str.to_string() }))
}
