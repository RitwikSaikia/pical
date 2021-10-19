use crate::{Agent, Action};

#[test]
#[allow(non_snake_case)]
fn wrap_a_closure() {
    let mut executed = 0;

    assert_eq!(0, executed);

    {
        let transition = Action!(|| {
            executed += 1;
        });
        match transition {
            Agent::Action(call) => {
                call.lock().unwrap()()
            }
            _ => panic!()
        }
    }

    assert_eq!(1, executed);
}

#[test]
#[allow(non_snake_case)]
fn prefix() {
    let _transition = Action!(|| {
            panic!()
        });
}
