pub use agent::*;
pub use prefix::*;
pub use channel::*;
pub use name::*;


#[macro_export]
#[allow(non_snake_case)]
macro_rules! NilAgent {
    () => {Agent::Nil};
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Agent {
    ($func:expr) => {
        {
            use std::sync::{Arc, Mutex};
            Agent::Restricted(Arc::new(Mutex::new($func)))
        }
    };
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Action {
    ($func:expr) => {
        {
            use std::sync::{Arc, Mutex};
            Agent::Action(Arc::new(Mutex::new($func)))
        }
    };
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! If {
    ($cond:expr, $then:expr) => {
        {
            use std::sync::Arc;
            Agent::Match(Arc::new(move || $cond), Arc::new($then))
        }
    };
}

mod agent;
mod prefix;
mod channel;
mod name;
