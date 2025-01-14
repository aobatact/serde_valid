use super::{ArrayErrors, ObjectErrors, VecErrors};

#[derive(Debug, Clone, serde::Serialize, thiserror::Error)]
#[serde(untagged)]
pub enum Errors {
    Array(ArrayErrors),
    Object(ObjectErrors),
    #[serde(serialize_with = "serialize")]
    NewType(VecErrors),
}

impl Errors {
    pub fn merge(&mut self, other: Errors) {
        match self {
            Errors::Array(a) => match other {
                Errors::Array(b) => {
                    a.errors.extend(b.errors);

                    for (index, item) in b.items {
                        match a.items.get_mut(&index) {
                            Some(errors) => errors.merge(item),
                            None => {
                                a.items.insert(index, item);
                            }
                        };
                    }
                }
                Errors::Object(_) => {
                    unreachable!("conflict Array and Object in serde_valid::validation::Errors")
                }
                Errors::NewType(errors) => {
                    a.errors.extend(errors.into_iter());
                }
            },
            Errors::NewType(a) => match other {
                Errors::Array(b) => {
                    a.extend(b.errors);
                    *self = Errors::Array(ArrayErrors::new(a.to_vec(), b.items));
                }
                Errors::Object(_) => {
                    unreachable!("conflict Array and Object in serde_valid::validation::Errors")
                }
                Errors::NewType(b) => {
                    a.extend(b);
                }
            },
            Errors::Object(_) => {
                unimplemented!("Object not support yet.")
            }
        }
    }
}

pub fn serialize<T>(errors: &VecErrors, serializer: T) -> Result<T::Ok, T::Error>
where
    T: serde::ser::Serializer,
{
    serializer.collect_map([("errors", errors)])
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Array(errors) => std::fmt::Display::fmt(errors, f),
            Self::Object(errors) => std::fmt::Display::fmt(errors, f),
            Self::NewType(vec_errors) => {
                match serde_json::to_string(
                    &vec_errors
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>(),
                ) {
                    Ok(json_string) => write!(f, "{}", json_string),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
