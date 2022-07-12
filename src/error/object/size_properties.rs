use crate::error::ToDefaultMessage;

macro_rules! struct_object_size_params {
    ($Params:tt, $limit:tt, $message:tt) => {
        #[derive(Debug, Clone)]
        pub struct $Params {
            pub $limit: usize,
        }

        impl $Params {
            pub fn new($limit: usize) -> Self {
                Self { $limit }
            }
        }

        impl ToDefaultMessage for $Params {
            fn to_default_message(&self) -> String {
                format!($message, self.$limit)
            }
        }
    };
}

struct_object_size_params!(
    MinPropertiesErrorParams,
    min_properties,
    "the size of the properties must be `>= {}`."
);
struct_object_size_params!(
    MaxPropertiesErrorParams,
    max_properties,
    "the size of the properties must be `<= {}`."
);
