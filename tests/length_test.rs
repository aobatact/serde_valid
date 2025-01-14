use serde_json::json;
use serde_valid::{Validate, ValidateMaxLength, ValidateMinLength};
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};

#[test]
fn length_string_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 4)]
        #[validate(max_length = 4)]
        val: String,
    }

    let s = TestStruct {
        val: String::from("test"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_str_type() {
    #[derive(Validate)]
    struct TestStruct<'a> {
        #[validate(min_length = 4)]
        #[validate(max_length = 4)]
        val: &'a str,
    }

    let s = TestStruct { val: "test" };
    assert!(s.validate().is_ok());
}

#[test]
fn length_cow_str_type() {
    #[derive(Validate)]
    struct TestStruct<'a> {
        #[validate(min_length = 4)]
        #[validate(max_length = 4)]
        val: Cow<'a, str>,
    }

    let s = TestStruct {
        val: Cow::from("test"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_os_str_type() {
    #[derive(Validate)]
    struct TestStruct<'a> {
        #[validate(min_length = 4)]
        #[validate(max_length = 4)]
        val: &'a OsStr,
    }

    let s = TestStruct {
        val: OsStr::new("fo�o"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_os_string_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 4)]
        #[validate(max_length = 4)]
        val: OsString,
    }

    let s = TestStruct {
        val: OsString::from("fo�o"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_path_type() {
    #[derive(Validate)]
    struct TestStruct<'a> {
        #[validate(min_length = 13)]
        #[validate(max_length = 13)]
        val: &'a std::path::Path,
    }

    let s = TestStruct {
        val: std::path::Path::new("./foo/bar.txt"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_path_buf_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 13)]
        #[validate(max_length = 13)]
        val: std::path::PathBuf,
    }

    let s = TestStruct {
        val: std::path::PathBuf::from("./foo/bar.txt"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_min_length_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 0)]
        #[validate(max_length = 10)]
        val: String,
    }

    let s = TestStruct { val: String::new() };
    assert!(s.validate().is_ok());
}

#[test]
fn length_min_length_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 1)]
        #[validate(max_length = 10)]
        val: String,
    }

    let s = TestStruct { val: String::new() };
    assert!(s.validate().is_err());
}

#[test]
fn length_max_length_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 0)]
        #[validate(max_length = 5)]
        val: String,
    }

    let s = TestStruct {
        val: String::from("abcde"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_max_length_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 1)]
        #[validate(max_length = 3)]
        val: String,
    }

    let s = TestStruct {
        val: String::from("abcd"),
    };
    assert!(s.validate().is_err());
}

#[test]
fn length_vec_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 0)]
        #[validate(min_length = 0)]
        val: Vec<String>,
    }

    let s = TestStruct {
        val: vec![String::from("abcd"), String::from("efg")],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_nested_vec_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 0)]
        #[validate(max_length = 3)]
        val: Vec<Vec<String>>,
    }

    let s = TestStruct {
        val: vec![
            vec![String::from(""), String::from("1")],
            vec![String::from("12"), String::from("123")],
        ],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_option_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 0)]
        #[validate(max_length = 5)]
        val: Option<String>,
    }

    let s = TestStruct {
        val: Some(String::from("abcd")),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_nested_option_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 0)]
        #[validate(max_length = 5)]
        val: Option<Option<String>>,
    }

    let s = TestStruct {
        val: Some(Some(String::from("abcd"))),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_vec_optional_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 0)]
        #[validate(max_length = 5)]
        val: Vec<Option<String>>,
    }

    let s = TestStruct {
        val: vec![Some(String::from("abc")), Some(String::from("abcde")), None],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 1)]
        #[validate(max_length = 3)]
        val: String,
    }

    let s = TestStruct {
        val: String::from("test"),
    };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "the length of the value must be `<= 3`."
                    ]
                }
            }
        }))
        .unwrap()
    );
}

#[test]
fn length_custom_err_message_fn() {
    fn custom_min_error_message(_params: &serde_valid::MinLengthErrorParams) -> String {
        "this is min custom message.".to_string()
    }

    fn custom_max_error_message(_params: &serde_valid::MaxLengthErrorParams) -> String {
        "this is max custom message.".to_string()
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 5, message_fn(custom_min_error_message))]
        #[validate(max_length = 3, message_fn(custom_max_error_message))]
        val: String,
    }

    let s = TestStruct {
        val: String::from("test"),
    };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "this is min custom message.",
                        "this is max custom message."
                    ]
                }
            }
        }))
        .unwrap()
    );
}

#[test]
fn length_custom_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 5, message = "this is min custom message.")]
        #[validate(max_length = 3, message = "this is max custom message.")]
        val: String,
    }

    let s = TestStruct {
        val: String::from("test"),
    };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "this is min custom message.",
                        "this is max custom message."
                    ]
                }
            }
        }))
        .unwrap()
    );
}

#[test]
fn length_vec_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 5, message = "this is min custom message.")]
        #[validate(max_length = 3, message = "this is max custom message.")]
        val: Vec<String>,
    }

    let s = TestStruct {
        val: vec![String::from("test")],
    };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": [
                                "this is min custom message.",
                                "this is max custom message."
                            ]
                        }
                    }
                }
            }
        }))
        .unwrap()
    );
}

#[test]
fn length_trait() {
    struct MyType(String);

    impl ValidateMaxLength for MyType {
        fn validate_max_length(
            &self,
            max_length: usize,
        ) -> Result<(), serde_valid::MaxLengthErrorParams> {
            self.0.validate_max_length(max_length)
        }
    }

    impl ValidateMinLength for MyType {
        fn validate_min_length(
            &self,
            min_length: usize,
        ) -> Result<(), serde_valid::MinLengthErrorParams> {
            self.0.validate_min_length(min_length)
        }
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 5)]
        #[validate(max_length = 5)]
        val: MyType,
    }

    let s = TestStruct {
        val: MyType(String::from("😍👺🙋🏽👨‍🎤👨‍👩‍👧‍👦")),
    };

    assert!(s.validate().is_ok());
}
