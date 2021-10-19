use std::sync::Arc;
use crate::{Channel, Name, Prefix};



#[test]
#[allow(non_snake_case)]
fn send_message() {
    let name = Name::new(123);
    let channel = Channel::new();

    let prefix = &channel << &name;
    match prefix {
        Prefix::Send(c, n) => {
            assert!(channel == c);
            assert!(Arc::ptr_eq(&n.value(), &name.value()))
        }
        _ => panic!()
    }
}

#[test]
#[allow(non_snake_case)]
fn recv_message() {
    let name = Name::new(123);
    let channel = Channel::new();

    let prefix = &channel >> &name;

    match prefix {
        Prefix::Receive(c, n) => {
            assert!(channel == c);
            assert!(Arc::ptr_eq(&n.value(), &name.value()))
        }
        _ => {},
    }
}


