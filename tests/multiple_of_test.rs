use serde_json::json;
use serde_valid::{Validate, ValidateMultipleOf};

#[test]
fn multiple_of_integer_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 5)]
        val: i32,
    }

    let s = TestStruct { val: 15 };
    assert!(s.validate().is_ok());
}

#[test]
fn multiple_of_float_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 1.0)]
        val: f32,
    }

    let s = TestStruct { val: 15.0 };
    assert!(s.validate().is_ok());
}

#[test]
fn multiple_of_integer_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 3)]
        val: i32,
    }

    let s = TestStruct { val: 16 };
    assert!(s.validate().is_err());
}

#[test]
fn multiple_of_float_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 0.5)]
        val: f32,
    }

    let s = TestStruct { val: 12.3 };
    assert!(s.validate().is_err());
}

#[test]
fn multiple_of_vec_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 4)]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![12, 16] };
    assert!(s.validate().is_ok());
}

#[test]
fn multiple_of_nested_vec_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 4)]
        val: Vec<Vec<i32>>,
    }

    let s = TestStruct {
        val: vec![vec![4, 8], vec![12, 16]],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn multiple_of_optional_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 4)]
        val: Option<i32>,
    }

    let s = TestStruct { val: Some(12) };
    assert!(s.validate().is_ok());
}

#[test]
fn multiple_of_nested_optional_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 4)]
        val: Option<Option<i32>>,
    }

    let s = TestStruct {
        val: Some(Some(12)),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn multiple_of_vec_optional_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 4)]
        val: Vec<Option<i32>>,
    }

    let s = TestStruct {
        val: vec![Some(4), Some(8), None],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn multiple_of_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 5)]
        val: i32,
    }

    let s = TestStruct { val: 14 };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "the value must be multiple of `5`."
                    ]
                }
            }
        }))
        .unwrap()
    );
}

#[test]
fn multiple_of_custom_err_message_fn() {
    fn error_message(_params: &serde_valid::MultipleOfErrorParams) -> String {
        "this is custom message.".to_string()
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 5, message_fn(error_message))]
        val: i32,
    }

    let s = TestStruct { val: 14 };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "this is custom message."
                    ]
                }
            }
        }))
        .unwrap()
    );
}

#[test]
fn multiple_of_custom_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 5, message = "this is custom message.")]
        val: i32,
    }

    let s = TestStruct { val: 14 };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "this is custom message."
                    ]
                }
            }
        }))
        .unwrap()
    );
}

#[test]
fn multiple_of_trait() {
    struct MyType(i32);

    impl ValidateMultipleOf<i32> for MyType {
        fn validate_multiple_of(
            &self,
            multiple_of: i32,
        ) -> Result<(), serde_valid::MultipleOfErrorParams> {
            self.0.validate_multiple_of(multiple_of)
        }
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 5)]
        val: MyType,
    }

    let s = TestStruct { val: MyType(10) };

    assert!(s.validate().is_ok());
}
