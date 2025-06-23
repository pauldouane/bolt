use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct Entry {
    value: String,
    pub(crate) expire_at: Option<SystemTime>,
}

impl Entry {
    pub(crate) fn new(value: String, ttl_secs: Option<u64>) -> Entry {
        let expire_at: Option<SystemTime> = if let Some(ttl_secs) = ttl_secs {
            Some(SystemTime::now() + Duration::from_secs(ttl_secs))
        } else { 
            None 
        };
        Entry {
            value,
            expire_at
        }
    }
}