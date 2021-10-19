use std::ops::Mul;
use std::sync::Arc;

use crate::{Agent, Channel, Name};

#[derive(Clone)]
pub enum Prefix {
    Silent,
    Send(Channel, Name),
    Receive(Channel, Name),
    Concat(Arc<Prefix>, Arc<Prefix>),

    #[cfg(test)] __Mock(String),
}

// Concat
impl<'a> Mul<Agent<'a>> for Agent<'a> {
    type Output = Agent<'a>;

    fn mul(self, rhs: Agent<'a>) -> Self::Output {
        Agent::ConcatAgent(Arc::new(self), Arc::new(rhs))
    }
}


impl<'a> Mul<Agent<'a>> for Prefix {
    type Output = Agent<'a>;

    fn mul(self, rhs: Agent<'a>) -> Self::Output {
        Agent::ConcatPrefix(Arc::new(self), Arc::new(rhs))
    }
}

impl<'a> Mul<Prefix> for Prefix {
    type Output = Prefix;

    fn mul(self, rhs: Prefix) -> Self::Output {
        Self::Concat(Arc::new(self), Arc::new(rhs))
    }
}

impl<'a> Mul<Arc<Prefix>> for Prefix {
    type Output = Prefix;

    fn mul(self, rhs: Arc<Prefix>) -> Self::Output {
        Self::Concat(Arc::new(self), rhs)
    }
}


impl<'a> Into<Agent<'a>> for Prefix {
    fn into(self) -> Agent<'a> {
        Agent::ConcatPrefix(Arc::new(self), Arc::new(Agent::Nil))
    }
}

#[cfg(test)]
impl Prefix {
    pub(crate) fn mock(&self) -> &str {
        match self {
            Prefix::__Mock(x) => x.as_str(),
            _ => panic!()
        }
    }
}