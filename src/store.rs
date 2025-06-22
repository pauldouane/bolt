use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub(crate) struct Store {
    data: RwLock<HashMap<String, String>>,
}

impl Store {
    pub(crate) fn new() -> Self {
        Store {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub(crate) fn set(&self, key: String, value: String) {
        let mut write_lock: RwLockWriteGuard<HashMap<String, String>> = self.data.write().unwrap();
        write_lock.insert(key, value);
    }

    pub(crate) fn get(&self, key: &str) -> Option<String> {
        let read_lock: RwLockReadGuard<HashMap<String, String>> = self.data.read().unwrap();
        read_lock.get(key).cloned()
    }

    fn delete(&self, key: &str) -> bool {
        let mut write_lock: RwLockWriteGuard<HashMap<String, String>> = self.data.write().unwrap();
        if !write_lock.contains_key(key) {
            return false;
        }
        write_lock.remove(key);
        true
    }

    fn keys(&self) -> Vec<String> {
        let read_lock: RwLockReadGuard<HashMap<String, String>> = self.data.read().unwrap();
        read_lock.keys().cloned().collect()
    }
}