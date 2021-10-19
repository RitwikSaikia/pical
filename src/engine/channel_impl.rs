use std::any::Any;
use std::ops::{Shl, Shr};
use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender};

use crate::Name;

pub(crate) struct ChannelImpl {
    tx: Sender<Arc<dyn Any + Send + Sync>>,
    rx: Receiver<Arc<dyn Any + Send + Sync>>,
}

impl ChannelImpl {
    pub fn new() -> ChannelImpl {
        let (tx, rx) = mpsc::channel();
        Self {
            tx,
            rx,
        }
    }
}

impl Shl<&Name> for &ChannelImpl {
    type Output = ();

    fn shl(self, rhs: &Name) -> Self::Output {
        self.tx.send(rhs.value()).unwrap();
    }
}

impl Shr<&Name> for &ChannelImpl {
    type Output = ();

    fn shr(self, rhs: &Name) -> Self::Output {
        let value = self.rx.recv().unwrap();
        let mut name = rhs.clone();
        name %= value;
    }
}