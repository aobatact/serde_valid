/// Max length validation of the array items.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
pub trait ValidateMaxItems {
    fn validate_max_items(&self, max_items: usize) -> Result<(), crate::MaxItemsErrorParams>;
}

impl<T> ValidateMaxItems for Vec<T> {
    fn validate_max_items(&self, max_items: usize) -> Result<(), crate::MaxItemsErrorParams> {
        if max_items >= self.len() {
            Ok(())
        } else {
            Err(crate::MaxItemsErrorParams::new(max_items))
        }
    }
}

impl<T, const N: usize> ValidateMaxItems for [T; N] {
    fn validate_max_items(&self, max_items: usize) -> Result<(), crate::MaxItemsErrorParams> {
        if max_items >= self.len() {
            Ok(())
        } else {
            Err(crate::MaxItemsErrorParams::new(max_items))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_vec_type() {
        assert!(ValidateMaxItems::validate_max_items(&vec!['a', 'b', 'c'], 3).is_ok());
    }

    #[test]
    fn test_validate_array_max_items_array_type() {
        assert!(ValidateMaxItems::validate_max_items(&['a', 'b', 'c'], 3).is_ok());
    }

    #[test]
    fn test_validate_array_max_items_is_true() {
        assert!(ValidateMaxItems::validate_max_items(&[1, 2, 3], 3).is_ok());
    }

    #[test]
    fn test_validate_array_max_items_is_false() {
        assert!(ValidateMaxItems::validate_max_items(&[1, 2, 3], 2).is_err());
    }
}
