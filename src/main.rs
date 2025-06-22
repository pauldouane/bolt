use crate::store::Store;

mod store;

fn main() {
    let mut store :Store = Store::new();
    store.set("key1".to_string(), "value1".to_string());
    println!("{}", store.get("key1").unwrap());
}
