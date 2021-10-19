use std::sync::Arc;
use crate::{Name};

#[test]
#[allow(non_snake_case)]
fn store_an_arbitrary_value() {
    let name = Name::new(123);

    let result = name.value();
    let result = result
        .downcast_ref::<i32>()
        .unwrap();

    assert_eq!(&123, result);
}


#[test]
#[allow(non_snake_case)]
fn is_mutable() {
    let mut name = Name::new(0);
    name %= Arc::new(123_i32);

    let result = name.value();
    let result = result
        .downcast_ref::<i32>()
        .unwrap();

    assert_eq!(&123, result);
}
