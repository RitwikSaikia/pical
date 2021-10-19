use std::hash::{Hash, Hasher};
use std::ops::{Shl, Shr};
use std::sync::Arc;

use crate::{Name, Prefix};

#[derive(Clone)]
pub struct Channel {
    ctx: Arc<usize>,
}

impl Channel {
    pub fn new() -> Self {
        Self {
            ctx: Arc::new(0)
        }
    }
}

impl Hash for Channel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hash = {
            let ptr = self.ctx.as_ref() as *const usize;
            ptr as usize
        };
        hash.hash(state);
    }
}

impl Shl<&Name> for &Channel {
    type Output = Prefix;

    fn shl(self, rhs: &Name) -> Self::Output {
        Prefix::Send(self.clone(), rhs.clone())
    }
}


impl Shr<&Name> for &Channel {
    type Output = Prefix;

    fn shr(self, rhs: &Name) -> Self::Output {
        Prefix::Receive(self.clone(), rhs.clone())
    }
}

impl PartialEq<Self> for Channel {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.ctx, &other.ctx)
    }
}

impl Eq for Channel {}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use crate::Channel;

    #[test]
    fn test() {
        let ch = Channel::new();

        let c1 = ch.clone();
        let c2 = ch.clone();

        assert!(ch == c1);
        assert!(ch == c2);

        let hash = compute_hash(&ch);

        assert_eq!(hash, compute_hash(&c1));
        assert_eq!(hash, compute_hash(&c2));
    }

    fn compute_hash(ch: &Channel) -> u64 {
        let hash = {
            let mut state = DefaultHasher::default();
            ch.hash(&mut state);
            state.finish()
        };
        hash
    }
}