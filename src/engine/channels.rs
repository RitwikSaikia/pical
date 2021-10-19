use std::collections::HashMap;
use std::sync::{Mutex};

use crate::{Channel, Name};
use crate::engine::ChannelImpl;

pub(crate) struct Channels {
    map: Mutex<HashMap<Channel, ChannelImpl>>,
}

impl Channels {
    pub fn new() -> Self {
        Self {
            map: Default::default()
        }
    }

    pub fn send(&self, channel: &Channel, name: &Name) {
        self.ready(channel);
        let lock = self.map.lock().unwrap();
        let map = &*lock;
        let imp = map.get(channel).unwrap();

        imp << name;
    }

    pub fn recv(&self, channel: &Channel, name: &Name) {
        self.ready(channel);
        let lock = self.map.lock().unwrap();
        let map = &*lock;
        let imp = map.get(channel).unwrap();

        imp >> name;
    }

    fn ready(&self, channel: &Channel) {
        let mut lock = self.map.lock().unwrap();
        let map = &mut *lock;
        if !map.contains_key(&channel) {
            let v = ChannelImpl::new();
            map.insert(channel.clone(), v);
        }
    }
}