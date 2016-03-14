use std::sync::RwLock;
use std::collections::BTreeMap;
use std::collections::Bound::{Included, Unbounded};

use storage::{Key, RefKey, Value, KvPair};
use util::HandyRwLock;
use super::{Engine, Modify, Result};

#[derive(Debug)]
pub struct EngineBtree {
    map: RwLock<BTreeMap<Key, Value>>,
}

impl EngineBtree {
    pub fn new() -> EngineBtree {
        info!("EngineBtree: creating");
        EngineBtree { map: RwLock::new(BTreeMap::new()) }
    }
}

impl Engine for EngineBtree {
    fn get(&self, key: RefKey) -> Result<Option<Value>> {
        trace!("EngineBtree: get {:?}", key);
        let m = self.map.rl();
        Ok(m.get(key).cloned())
    }

    fn seek(&self, key: RefKey) -> Result<Option<KvPair>> {
        trace!("EngineBtree: seek {:?}", key);
        let m = self.map.rl();
        let mut iter = m.range::<Key, Key>(Included(&key.to_vec()), Unbounded);
        Ok(iter.next().map(|(k, v)| (k.clone(), v.clone())))
    }

    fn write(&self, batch: Vec<Modify>) -> Result<()> {
        let mut m = self.map.wl();
        for rev in batch {
            match rev {
                Modify::Delete(k) => {
                    trace!("EngineBtree: delete {:?}", k);
                    m.remove(&k);
                }
                Modify::Put((k, v)) => {
                    trace!("EngineBtree: put {:?},{:?}", k, v);
                    m.insert(k, v);
                }
            }
        }
        Ok(())
    }
}
