use crate::helper::NamedField;
use crate::lit::LitNumber;
use crate::validator::{abort_invalid_attribute_on_field, Validator};
use quote::quote;

pub fn extract_multiples_validator(field: &NamedField, lit: &syn::Lit) -> Validator {
    let field_ident = field.ident();
    let multiple_of = match lit {
        syn::Lit::Int(l) => LitNumber::Int(l.to_owned()),
        syn::Lit::Float(l) => LitNumber::Float(l.to_owned()),
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
            "invalid argument type for `multiple_of` validator: only number literals are allowed",
        ),
    };
    let token = quote!(
        if !::serde_valid::validate_multiples(
            #field_ident,
            #multiple_of,
        ) {
            errors.push(::serde_valid::Error::MultipleOfError);
        }
    );
    Validator::Normal(token)
}