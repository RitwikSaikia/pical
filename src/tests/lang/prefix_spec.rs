use crate::{Agent, Prefix};

#[allow(non_snake_case)]
struct Test {
    Q: Prefix,
    R: Prefix,
    S: Prefix,
}

derive_mock_accessor!(Q, Prefix);
derive_mock_accessor!(R, Prefix);
derive_mock_accessor!(S, Prefix);

impl Default for Test {
    fn default() -> Self {
        Self {
            Q: Prefix::__Mock("Q".to_string()),
            R: Prefix::__Mock("R".to_string()),
            S: Prefix::__Mock("S".to_string()),
        }
    }
}

#[test]
#[allow(non_snake_case)]
fn concatenation_of_prefixes() {
    let test: Test = Default::default();

    let Q = test.Q.clone();
    let R = test.R.clone();
    let S = test.S.clone();

    let P = Q * R * S;

    match P {
        Prefix::Concat(prefix, agent) => {
            match prefix.as_ref() {
                Prefix::Concat(x, y) => {
                    assert_eq!(test.Q(), x.as_ref().mock());
                    assert_eq!(test.R(), y.as_ref().mock());
                }
                _ => panic!(),
            }
            assert_eq!(test.S(), agent.mock());
        }
        _ => panic!()
    }
}


#[test]
#[allow(non_snake_case)]
fn convert_to_agent() {
    let test: Test = Default::default();

    let Q = test.Q.clone();

    let P: Agent = Q.into();

    match P {
        Agent::ConcatPrefix(x, y) => {
            assert_eq!(test.Q(), x.mock());
            match y.as_ref() {
                Agent::Nil => {}
                _ => panic!(),
            }
        }
        _ => panic!()
    }
}
