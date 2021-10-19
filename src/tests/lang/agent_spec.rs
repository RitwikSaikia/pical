use std::sync::Arc;
use crate::{Agent, Channel, Name, Prefix};

#[allow(non_snake_case)]
struct Test<'a> {
    Q: Prefix,
    R: Prefix,
    S: Agent<'a>,
    T: Agent<'a>,
}

derive_mock_accessor!(Q, Prefix, 'a);
derive_mock_accessor!(R, Prefix, 'a);
derive_mock_accessor!(S, Agent, 'a);
derive_mock_accessor!(T, Agent, 'a);

impl<'a> Default for Test<'a> {
    fn default() -> Self {
        Self {
            Q: Prefix::__Mock("Q".to_string()),
            R: Prefix::__Mock("R".to_string()),
            S: Agent::__Mock("S".to_string()),
            T: Agent::__Mock("T".to_string()),
        }
    }
}

#[test]
#[allow(non_snake_case)]
fn concatenation_of_prefixes_followed_by_an_agent() {
    let test: Test = Default::default();

    let Q = test.Q.clone();
    let R = test.R.clone();
    let S = test.S.clone();

    let P = Q * R * S;

    match P {
        Agent::ConcatPrefix(prefix, agent) => {
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
fn have_a_composition_operator() {
    let test: Test = Default::default();

    let S = test.S.clone();
    let T = test.T.clone();

    let P = S.clone() | T | S;

    match P {
        Agent::Compose(x, y) => {
            match x.as_ref() {
                Agent::Compose(x, y) => {
                    assert_eq!(test.S(), x.mock());
                    assert_eq!(test.T(), y.mock());
                }
                _ => panic!()
            }
            assert_eq!(test.S(), y.mock());
        }
        _ => panic!(),
    }
}

#[test]
#[allow(non_snake_case)]
fn concatenation_has_precedence_over_composition() {
    let test: Test = Default::default();

    let Q = test.Q.clone();
    let R = test.R.clone();
    let S = test.S.clone();
    let T = test.T.clone();

    let P = Q * S | R * T;

    match P {
        Agent::Compose(x, y) => {
            match x.as_ref() {
                Agent::ConcatPrefix(x, y) => {
                    assert_eq!(test.Q(), x.mock());
                    assert_eq!(test.S(), y.mock());
                }
                _ => panic!()
            }

            match y.as_ref() {
                Agent::ConcatPrefix(x, y) => {
                    assert_eq!(test.R(), x.mock());
                    assert_eq!(test.T(), y.mock());
                }
                _ => panic!()
            }
        }
        _ => panic!()
    }
}

#[test]
#[allow(non_snake_case)]
fn guarded_by_a_prefix() {
    let test: Test = Default::default();

    let Q = test.Q.clone();
    let S = test.S.clone();

    let P = Q ^ S;

    match P {
        Agent::GuardPrefix(prefix, agent) => {
            assert_eq!(test.Q(), prefix.mock());
            assert_eq!(test.S(), agent.mock());
        }
        _ => panic!(),
    }
}

#[test]
#[allow(non_snake_case)]
fn has_a_sum_operator() {
    let test: Test = Default::default();

    let Q = test.Q.clone();
    let R = test.R.clone();
    let S = test.S.clone();
    let T = test.T.clone();

    let G1 = Q.clone() ^ S;
    let G2 = R.clone() ^ T.clone();
    let G3 = Q ^ R * T;

    let P = G1 + G2 + G3;

    match P {
        Agent::Sum(g1g2, g3) => {
            match g1g2.as_ref() {
                Agent::Sum(g1, g2) => {
                    match g1.as_ref() {
                        Agent::GuardPrefix(prefix, agent) => {
                            assert_eq!(test.Q(), prefix.mock());
                            assert_eq!(test.S(), agent.mock());
                        }
                        _ => panic!()
                    }
                    match g2.as_ref() {
                        Agent::GuardPrefix(prefix, agent) => {
                            assert_eq!(test.R(), prefix.mock());
                            assert_eq!(test.T(), agent.mock());
                        }
                        _ => panic!()
                    }
                }
                _ => panic!()
            }
            match g3.as_ref() {
                Agent::GuardPrefix(prefix, agent) => {
                    assert_eq!(test.Q(), prefix.mock());
                    match agent.as_ref() {
                        Agent::ConcatPrefix(x, y) => {
                            assert_eq!(test.R(), x.mock());
                            assert_eq!(test.T(), y.mock());
                        }
                        _ => panic!()
                    }
                }
                _ => panic!()
            }
        }
        _ => panic!()
    }
}


#[test]
#[allow(non_snake_case)]
fn restrict_names_to_agent_instances() {
    let P = Agent!(|| {
        let n = Name::new(true);
        let c = Channel::new();
        (&c << &n).into()
    });

    let agent = match P {
        Agent::Restricted(v) => v,
        _ => panic!()
    };
    let a = agent.lock().unwrap()();
    let b = agent.lock().unwrap()();

    let a_prefix = match &a {
        Agent::ConcatPrefix(prefix, agent) => {
            match agent.as_ref() {
                Agent::Nil => {}
                _ => panic!()
            }
            prefix
        }
        _ => panic!()
    };

    let b_prefix = match &b {
        Agent::ConcatPrefix(prefix, agent) => {
            match agent.as_ref() {
                Agent::Nil => {}
                _ => panic!()
            }
            prefix
        }
        _ => panic!()
    };

    match (a_prefix.as_ref(), b_prefix.as_ref()) {
        (Prefix::Send(a_channel, a_name),
            Prefix::Send(b_channel, b_name)) => {
            assert!(a_channel != b_channel);
            assert!(!Arc::ptr_eq(&a_name.value(), &b_name.value()));
        }
        (_, _) => panic!(),
    }
}