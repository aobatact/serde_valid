use crate::types::Field;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_validator_from_meta_path(field: &impl Field) -> Result<Validator, crate::Errors> {
    let validator = if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_validator_from_meta_path(&array_field)?))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_validator_from_meta_path(&option_field)?))
    } else {
        Validator::Normal(inner_extract_validator_from_meta_path(field))
    };

    Ok(validator)
}

fn inner_extract_validator_from_meta_path(field: &impl Field) -> TokenStream {
    let field_ident = field.ident();
    let field_name = field.name();
    quote!(
        if let Err(__inner_errors) = #field_ident.validate() {
            match __inner_errors {
                __fields_errors @ ::serde_valid::validation::Errors::Fields(_) => {
                    __errors.insert(
                        #field_name,
                        vec![::serde_valid::validation::Error::Nested(__fields_errors)]
                    );
                }
                ::serde_valid::validation::Errors::NewType(__new_type_errors) => {
                    __errors.insert(#field_name, __new_type_errors);
                }
            }
        }
    )
}