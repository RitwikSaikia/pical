use std::ops::{Add, BitOr, BitXor};
use std::sync::{Arc, Mutex};
use crate::Prefix;

#[derive(Clone)]
pub enum Agent<'a> {
    Nil,
    Restricted(Arc<Mutex<dyn FnMut() -> Agent<'a> + 'a + Sync + Send>>),
    Action(Arc<Mutex<dyn FnMut() + 'a + Sync + Send>>),
    ConcatPrefix(Arc<Prefix>, Arc<Agent<'a>>),
    ConcatAgent(Arc<Agent<'a>>, Arc<Agent<'a>>),
    Compose(Arc<Agent<'a>>, Arc<Agent<'a>>),
    Sum(Arc<Agent<'a>>, Arc<Agent<'a>>),
    GuardPrefix(Arc<Prefix>, Arc<Agent<'a>>),
    GuardAgent(Arc<Agent<'a>>, Arc<Agent<'a>>),
    Match(Arc<dyn Fn() -> bool + 'a + Sync + Send>,
          Arc<dyn Fn() -> Agent<'a> + 'a + Sync + Send>),

    #[cfg(test)] __Mock(String),
}

// Compose
impl<'a> BitOr<Agent<'a>> for Agent<'a> {
    type Output = Agent<'a>;

    fn bitor(self, rhs: Agent<'a>) -> Self::Output {
        Self::Compose(Arc::new(self), Arc::new(rhs))
    }
}


// Summation
impl<'a> Add<Agent<'a>> for Agent<'a> {
    type Output = Agent<'a>;

    fn add(self, rhs: Agent<'a>) -> Self::Output {
        match &self {
            Agent::Sum(_, _) => {}
            Agent::GuardPrefix(_, _) => {}
            Agent::GuardAgent(_, _) => {}
            _ => {
                panic!("Summation allowed only for guard or sum");
            }
        }
        match &rhs {
            Agent::Sum(_, _) => {}
            Agent::GuardPrefix(_, _) => {}
            Agent::GuardAgent(_, _) => {}
            _ => {
                panic!("Summation allowed only for guard or sum");
            }
        }
        Self::Sum(Arc::new(self), Arc::new(rhs))
    }
}


// Guard
impl<'a> BitXor<Agent<'a>> for Prefix {
    type Output = Agent<'a>;

    fn bitxor(self, rhs: Agent<'a>) -> Self::Output {
        Agent::GuardPrefix(Arc::new(self), Arc::new(rhs))
    }
}

impl<'a> BitXor<Agent<'a>> for Agent<'a> {
    type Output = Agent<'a>;

    fn bitxor(self, rhs: Agent<'a>) -> Self::Output {
        Agent::GuardAgent(Arc::new(self), Arc::new(rhs))
    }
}



#[cfg(test)]
impl<'a> Agent<'a> {
    pub(crate) fn mock(&self) -> &str {
        match self {
            Agent::__Mock(x) => x.as_str(),
            _ => panic!()
        }
    }
}
