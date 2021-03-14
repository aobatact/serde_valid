use serde_valid::Validate;

#[test]
fn range_integer_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0, maximum = 2000))]
        val: i32,
    }

    let s = TestStruct { val: 1234 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_float_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0.0, maximum = 2000.0))]
        val: f32,
    }

    let s = TestStruct { val: 1234.5678 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_exclusive_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(exclusive_minimum = 0, exclusive_maximum = 10))]
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_minimum_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0, maximum = 10))]
        val: i32,
    }

    let s = TestStruct { val: 0 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_minimum_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 1, maximum = 10))]
        val: i32,
    }

    let s = TestStruct { val: 0 };
    assert!(s.validate().is_err());
}

#[test]
fn range_exclusive_minimum_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(exclusive_minimum = 0, maximum = 10))]
        val: i32,
    }

    let s = TestStruct { val: 1 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_exclusive_minimum_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(exclusive_minimum = 0, maximum = 10))]
        val: i32,
    }

    let s = TestStruct { val: 0 };
    assert!(s.validate().is_err());
}

#[test]
fn range_maximum_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0, maximum = 10))]
        val: i32,
    }

    let s = TestStruct { val: 10 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_maximum_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 1, maximum = 10))]
        val: i32,
    }

    let s = TestStruct { val: 11 };
    assert!(s.validate().is_err());
}

#[test]
fn range_exclusive_maximum_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0, exclusive_maximum = 10))]
        val: i32,
    }

    let s = TestStruct { val: 9 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_exclusive_maximum_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0, exclusive_maximum = 10))]
        val: i32,
    }

    let s = TestStruct { val: 10 };
    assert!(s.validate().is_err());
}

#[test]
fn range_array_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0, maximum = 20))]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![12, 16] };
    assert!(s.validate().is_ok());
}

#[test]
fn range_nested_array_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0, maximum = 20))]
        val: Vec<Vec<i32>>,
    }

    let s = TestStruct {
        val: vec![vec![4, 8], vec![12, 16]],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn range_option_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0, maximum = 10))]
        val: Option<i32>,
    }

    let s = TestStruct { val: Some(5) };
    assert!(s.validate().is_ok());
}

#[test]
fn range_nested_option_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0, maximum = 10))]
        val: Option<Option<i32>>,
    }

    let s = TestStruct { val: Some(Some(5)) };
    assert!(s.validate().is_ok());
}

#[test]
fn range_array_optional_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0, maximum = 10))]
        val: Vec<Option<i32>>,
    }

    let s = TestStruct {
        val: vec![Some(4), Some(8), None],
    };
    assert!(s.validate().is_ok());
}