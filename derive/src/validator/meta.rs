mod meta_path;
mod nested_meta_list;
mod nested_meta_name_value;
mod nested_meta_path;

use crate::errors::Error;
use crate::types::Field;
use crate::validator::Validator;
use meta_path::extract_validator_from_meta_path;
use nested_meta_list::extract_validator_from_nested_meta_list;
use nested_meta_name_value::extract_validator_from_nested_meta_name_value;
use nested_meta_path::extract_validator_from_nested_meta_path;
use syn::spanned::Spanned;

pub fn extract_meta_validator(
    field: &impl Field,
    attribute: &syn::Attribute,
) -> Result<Validator, Error> {
    match attribute.parse_meta() {
        Ok(syn::Meta::List(syn::MetaList { ref nested, .. })) => {
            // only validation from there on
            if nested.len() > 0 {
                let meta_item = &nested[0];
                match meta_item {
                    syn::NestedMeta::Meta(item) => match item {
                        syn::Meta::Path(validation_path) => {
                            return extract_validator_from_nested_meta_path(
                                field,
                                attribute,
                                validation_path,
                            )
                        }
                        syn::Meta::List(validation_list) => {
                            return extract_validator_from_nested_meta_list(
                                field,
                                attribute,
                                validation_list,
                            )
                        }
                        syn::Meta::NameValue(validation_name_value) => {
                            return extract_validator_from_nested_meta_name_value(
                                field,
                                attribute,
                                validation_name_value,
                            )
                        }
                    },
                    syn::NestedMeta::Lit(_) => {
                        return Err(Error::new_literal_meta_item_error(meta_item.span()))
                    }
                };
            }
        }
        Ok(syn::Meta::Path(_)) => return extract_validator_from_meta_path(field, attribute),
        Ok(syn::Meta::NameValue(_)) => {
            return Err(Error::new_meta_name_value_item_error(attribute.span()))
        }
        Err(error) => return Err(Error::new_attribute_parse_error(attribute.span(), &error)),
    };

    Err(Error::new_attribute_required_error(attribute.span()))
}
