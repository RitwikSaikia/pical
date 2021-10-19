use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{Action, Agent, Channel, Name};
use crate::engine::Engine;

#[test]
fn run_actions() {
    let mut executed = false;

    let agent = Action!(|| {
            executed = true;
        });

    run_agent(agent);

    assert_eq!(true, executed);
}

#[test]
fn concatenation_agents() {
    let mut executed1 = 0;
    let mut executed2 = 0;
    let mut executed3 = 0;

    let exec_order = AtomicUsize::new(1);

    let action1 = Action!(|| {
            executed1 = exec_order.fetch_add(1, Ordering::SeqCst);
        });

    let action2 = Action!(|| {
            executed2 = exec_order.fetch_add(1, Ordering::SeqCst);
        });

    let action3 = Action!(|| {
            executed3 = exec_order.fetch_add(1, Ordering::SeqCst);
        });

    let agent = Agent!(move || {
            action1.clone() * action2.clone() * action3.clone()
        });

    run_agent(agent);


    assert_eq!(1, executed1);
    assert_eq!(2, executed2);
    assert_eq!(3, executed3);
}

#[test]
fn guarded_agents() {
    let mut executed1 = 0;
    let mut executed2 = 0;
    let mut executed3 = 0;

    let exec_order = AtomicUsize::new(1);

    let action1 = Action!(|| {
            executed1 = exec_order.fetch_add(1, Ordering::SeqCst);
        });

    let action2 = Action!(|| {
            executed2 = exec_order.fetch_add(1, Ordering::SeqCst);
        });

    let action3 = Action!(|| {
            executed3 = exec_order.fetch_add(1, Ordering::SeqCst);
        });

    let agent = Agent!(move || {
            action1.clone() ^ action2.clone() ^ action3.clone()
        });

    run_agent(agent);

    assert_eq!(1, executed1);
    assert_eq!(2, executed2);
    assert_eq!(3, executed3);
}

#[test]
fn parallel_agents() {
    let mut executed1 = false;
    let mut executed2 = false;
    let mut executed3 = false;

    let action1 = Action!(|| {
            executed1 = true;
        });

    let action2 = Action!(|| {
            executed2 = true;
        });

    let action3 = Action!(|| {
            executed3 = true;
        });

    let agent = Agent!(move || {
            action1.clone() | action2.clone() | action3.clone()
        });
    run_agent(agent);

    assert!(executed1);
    assert!(executed2);
    assert!(executed3);
}


#[test]
fn sum_agents() {
    let mut executed1 = 0;
    let mut executed2 = 0;
    let mut executed3 = 0;

    let mut executed_agent = false;

    let action1 = Action!(|| {
            executed1 = 1;
        });

    let action2 = Action!(|| {
            executed2 = 1;
        });

    let action3 = Action!(|| {
            executed3 = 1;
        });

    let action_agent = Action!(|| {
            executed_agent = true;
        });

    let agent = Agent!(move || {
                (action1.clone() ^ action_agent.clone()) +
                    (action2.clone() ^ action_agent.clone()) +
                    (action3.clone() ^ action_agent.clone())
        });
    run_agent(agent);

    assert_eq!(1, executed1 + executed2 + executed3);
    assert!(executed_agent);
}

#[test]
fn send_and_receive_messages() {
    let name2send = Name::new(123_i32);
    let name2recv = Name::new(456_i32);

    let channel = Channel::new();

    let agent = Agent!(|| {
        let sender: Agent = (&channel << &name2send).into();
        let receiver: Agent = (&channel >> &name2recv).into();
        sender | receiver
    });
    run_agent(agent);

    let a = name2send.value();
    let a = a.as_ref().downcast_ref::<i32>().unwrap();
    let b = name2recv.value();
    let b = b.as_ref().downcast_ref::<i32>().unwrap();

    assert_eq!(a, b);
    assert_eq!(&123, a);
    assert_eq!(&123, b);
}

fn run_agent(agent: Agent) {
    let mut engine = Engine::new(agent);
    engine.start();
    engine.stop();
}
