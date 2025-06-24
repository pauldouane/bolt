use crate::entry::Entry;
use crate::core::ReplOutput;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::alloc::System;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::process::Output;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, SystemTime, SystemTimeError};

const MAGIC_NUMBER: &[u8] = b"BOLT"; // 4 octets ASCII
const DATA_FILE: &str = "data.bolt";

#[derive(Serialize, Deserialize)]
pub(crate) struct Store {
    data: RwLock<HashMap<String, Entry>>,
}

impl Store {
    pub(crate) fn new() -> Self {
        Store::load_from_file(DATA_FILE).unwrap_or_else(|_err| Store {
            data: RwLock::new(HashMap::new()),
        })
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = BufReader::new(File::open(path)?);

        let mut magic = [0u8; 4];
        file.read_exact(&mut magic)?;
        if &magic != MAGIC_NUMBER {
            return Err("Invalid file format: wrong magic number".into());
        }

        let mut json_types: Vec<u8> = Vec::new();
        file.read_to_end(&mut json_types)?;

        let map: HashMap<String, Entry> = serde_json::from_slice(&json_types)?;
        Ok(Store {
            data: RwLock::new(map),
        })
    }

    pub(crate) fn from_hashmap(data: HashMap<String, Entry>) -> Self {
        Store {
            data: RwLock::new(data),
        }
    }

    pub(crate) fn set(&self, key: String, value: Entry) {
        let mut write_lock: RwLockWriteGuard<HashMap<String, Entry>> = self.data.write().unwrap();
        write_lock.insert(key, value);
    }

    pub(crate) fn get(&self, key: &str) -> Option<Entry> {
        let entry_opt = {
            let read_lock = self.data.read().unwrap();
            read_lock.get(key).cloned()
        };
        match entry_opt {
            Some(entry) => match Store::check_ttl(&entry) {
                Ok(entry_is_valid) => {
                    if entry_is_valid {
                        Some(entry)
                    } else {
                        self.delete(key);
                        None
                    }
                }
                Err(e) => panic!("TTL error: {}", e),
            },
            None => None,
        }
    }

    pub(crate) fn delete(&self, key: &str) -> bool {
        let mut write_lock = self.data.write().unwrap();
        if !write_lock.contains_key(key) {
            return false;
        }
        write_lock.remove(key);
        true
    }

    fn delete_given_keys(&self, keys: &Vec<String>) {
        for key in keys {
            self.delete(&key);
        }
    }

    pub(crate) fn keys(&self, output: &ReplOutput) {
        let read_lock: RwLockReadGuard<HashMap<String, Entry>> = self.data.read().unwrap();
        read_lock
            .keys()
            .for_each(|k| output.repl_write_line(format!("\r{}", k)).unwrap());
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        self.purge_expired();
        let map: RwLockReadGuard<HashMap<String, Entry>> = self.data.read().unwrap();
        let binary_data: Vec<u8> = serde_json::to_vec(&*map)?;

        let mut file = File::create(DATA_FILE)?;
        file.write_all(MAGIC_NUMBER)?;
        file.write_all(&binary_data)?;
        Ok(())
    }

    pub fn check_ttl(entry: &Entry) -> Result<bool, SystemTimeError> {
        match entry.expire_at {
            None => Ok(true),
            Some(expire_at) => {
                let now = SystemTime::now();
                if now > expire_at {
                    return Ok(false);
                }
                Ok(true)
            }
        }
    }

    pub fn purge_expired(&self) {
        let entry_opt: Vec<String> = {
            let read_lock = self.data.read().unwrap();
            read_lock
                .iter()
                .filter_map(|(k, v)| match Store::check_ttl(&v) {
                    Ok(entry_is_valid) => {
                        if entry_is_valid {
                            None
                        } else {
                            Some(k.clone())
                        }
                    }
                    Err(e) => panic!("TTL error: {}", e),
                })
                .collect()
        };
        self.delete_given_keys(&entry_opt);
    }
}
