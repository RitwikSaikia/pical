use crate::{Agent, If};

#[allow(non_snake_case)]
struct Test<'a> {
    Q: Agent<'a>,
}

derive_mock_accessor!(Q, Agent, 'a);

impl<'a> Default for Test<'a> {
    fn default() -> Self {
        Self {
            Q: Agent::__Mock("Q".to_string()),
        }
    }
}

#[test]
#[allow(non_snake_case)]
fn conditional_execution_of_an_agent() {
    let test: Test = Default::default();

    let Q = test.Q.clone();

    let P = If!(1 > 0, move || {Q.clone()});

    match P {
        Agent::Match(cond, then) => {
            assert_eq!(true, cond());
            assert_eq!(test.Q(), then().mock());
        }
        _ => panic!()
    }
}
