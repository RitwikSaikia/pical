use std::any::Any;
use std::ops::RemAssign;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Name {
    storage: Arc<Mutex<Arc<dyn Any + Send + Sync>>>,
}

impl Name {
    pub fn new(value: impl Any + Send + Sync) -> Self {
        Self {
            storage: Arc::new(Mutex::new(Arc::new(value)))
        }
    }

    pub fn value(&self) -> Arc<dyn Any + Send + Sync> {
        let lock = self.storage.lock().unwrap();
        lock.clone()
    }
}

impl RemAssign<Arc<dyn Any + Send + Sync>> for Name {
    fn rem_assign(&mut self, rhs: Arc<dyn Any + Send + Sync>) {
        let mut lock = self.storage.lock().unwrap();
        *lock = rhs;
    }
}
