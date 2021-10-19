use std::borrow::BorrowMut;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::{ThreadPool, ThreadPoolBuilder};

use crate::{Agent, Prefix};
use crate::engine::Channels;

pub struct Engine<'a> {
    agent: Agent<'a>,
    pool: ThreadPool,
    channels: Channels,
}

impl<'a> Engine<'a> {
    pub fn new(agent: Agent<'a>) -> Engine<'a> {
        Self {
            agent,
            pool: ThreadPoolBuilder::new()
                .build()
                .unwrap(),
            channels: Channels::new(),
        }
    }

    pub fn start(&mut self) {
        self.execute_in_thread(&self.agent);
    }

    pub fn stop(&mut self) {}

    fn execute_in_thread(&self, agent: &Agent<'a>) {
        self.pool.install(|| {
            self.execute_agent(agent);
        });
    }

    #[allow(unreachable_patterns)]
    fn execute_agent(&self, agent: &Agent<'a>) {
        match agent.clone() {
            Agent::Nil => {
                return;
            }
            Agent::Restricted(procedure) => {
                let mut lock = procedure.lock().unwrap();
                let procedure = lock();
                self.execute_agent(&procedure);
                return;
            }
            Agent::Action(procedure) => {
                let mut lock = procedure.lock().unwrap();
                lock();
                return;
            }
            Agent::ConcatPrefix(left, right) => {
                self.execute_prefix(left.as_ref());
                self.execute_agent(right.as_ref());
                return;
            }
            Agent::ConcatAgent(left, right) => {
                self.execute_agent(left.as_ref());
                self.execute_agent(right.as_ref());
                return;
            }
            Agent::Compose(left, right) => {
                self.execute_in_thread(left.as_ref());
                self.execute_in_thread(right.as_ref());
                return;
            }
            Agent::Match(mut cond, mut then) => {
                if cond.borrow_mut()() {
                    let procedure = then.borrow_mut()();
                    self.execute_agent(&procedure);
                }
                return;
            }
            Agent::GuardPrefix(left, right) => {
                self.execute_prefix(left.as_ref());
                self.execute_agent(right.as_ref());
                return;
            }
            Agent::GuardAgent(left, right) => {
                self.execute_agent(left.as_ref());
                self.execute_agent(right.as_ref());
                return;
            }
            Agent::Sum(left, right) => {
                let mut agents = self.sum_terms(agent);
                agents.shuffle(&mut thread_rng());
                let mut cont: Option<Agent> = None;
                while cont.is_none() {
                    for a in &agents {
                        if cont.is_none() {
                            cont = match a {
                                Agent::Action(a) => {
                                    a.lock().unwrap()();
                                    Some(right.as_ref().clone())
                                }
                                Agent::GuardAgent(left, right) => {
                                    self.execute_agent(left.as_ref());
                                    self.execute_agent(right.as_ref());
                                    Some(right.as_ref().clone())
                                }

                                Agent::GuardPrefix(left, right) => {
                                    self.execute_prefix(left.as_ref());
                                    self.execute_agent(right.as_ref());
                                    Some(right.as_ref().clone())
                                }
                                _ => {
                                    panic!()
                                }
                            }
                        }
                    }
                }
                self.execute_agent(&cont.unwrap());
                return;
            }

            _ => panic!()
        }
    }

    fn execute_prefix(&self, prefix: &Prefix) {
        #[allow(unreachable_patterns)]
        match prefix {
            Prefix::Silent => {
                return;
            }
            Prefix::Send(channel, name) => {
                self.channels.send(channel, name);
                return;
            }
            Prefix::Receive(channel, name) => {
                self.channels.recv(channel, name);
                return;
            }
            Prefix::Concat(left, right) => {
                self.execute_prefix(left.as_ref());
                self.execute_prefix(right.as_ref());
                return;
            }
            _ => panic!()
        }
    }

    fn sum_terms(&self, agent: &Agent<'a>) -> Vec<Agent<'a>> {
        match &agent {
            Agent::Sum(left, right) => {
                let mut left = self.sum_terms(left.as_ref());
                let right = self.sum_terms(right.as_ref());
                left.extend(right);
                left
            }
            Agent::GuardPrefix(..) => {
                vec![agent.clone()]
            }
            Agent::GuardAgent(..) => {
                vec![agent.clone()]
            }
            _ => {
                panic!()
            }
        }
    }
}