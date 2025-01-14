use crate::types::Field;
use crate::validate::common::get_numeric;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
macro_rules! extract_array_length_validator{
    ($ErrorType:ident) => {
        paste::paste! {
            pub fn [<extract_array_ $ErrorType:snake _validator>](
                field: &impl Field,
                validation_value: &syn::Lit,
                message_fn: Option<TokenStream>,
                rename_map: &HashMap<String, String>,
            ) -> Result<Validator, crate::Errors> {
                Ok([<inner_extract_array_ $ErrorType:snake _validator>](field, validation_value, message_fn, rename_map)?)
            }

            fn [<inner_extract_array_ $ErrorType:snake _validator>](
                field: &impl Field,
                validation_value: &syn::Lit,
                message_fn: Option<TokenStream>,
                rename_map: &HashMap<String, String>,
            ) -> Result<TokenStream, crate::Errors> {
                let field_name = field.name();
                let field_ident = field.ident();
                let rename = rename_map.get(field_name).unwrap_or(field_name);
                let [<$ErrorType:snake>] = get_numeric(validation_value)?;
                let message =
                    message_fn.unwrap_or(quote!(::serde_valid::[<$ErrorType ErrorParams>]::to_default_message));

                Ok(quote!(
                    if let Err(error_params) = ::serde_valid::[<Validate $ErrorType>]::[<validate_ $ErrorType:snake>](
                        #field_ident,
                        #[<$ErrorType:snake>],
                    ) {
                        use ::serde_valid::error::ToDefaultMessage;
                        __properties_errors
                            .entry(#rename)
                            .or_default()
                            .push(::serde_valid::validation::Error::$ErrorType(
                                ::serde_valid::error::Message::new(
                                    error_params,
                                    #message
                                )
                            ));
                    }
                ))
            }
        }
    }
}

extract_array_length_validator!(MaxItems);
extract_array_length_validator!(MinItems);
