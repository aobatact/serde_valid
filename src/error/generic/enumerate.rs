use itertools::Itertools;

use crate::{error::ToDefaultMessage, validation::Literal};

#[derive(Debug, Clone)]
pub struct EnumerateErrorParams {
    pub enumerate: Vec<Literal>,
}

impl EnumerateErrorParams {
    pub fn new<T>(enumerate: &[T]) -> Self
    where
        T: Into<Literal> + std::fmt::Debug + Clone,
    {
        Self {
            // FIXME: remove clone.
            enumerate: (*enumerate).iter().map(|x| x.clone().into()).collect(),
        }
    }
}

impl ToDefaultMessage for EnumerateErrorParams {
    fn to_default_message(&self) -> String {
        format!(
            "the value must be in [{:}].",
            self.enumerate.iter().map(|v| format!("{}", v)).join(", ")
        )
    }
}
