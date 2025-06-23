use std::time::SystemTime;
use crate::entry::Entry;
use crate::store::Store;

mod store;
mod entry;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store :Store = Store::new();
    match store.get("key2") {
        Some(entry) => println!("{:?}", entry),
        None => println!("No entry found")
    }
    match store.save_to_file() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into())
    }
}
