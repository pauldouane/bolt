use crate::entry::Entry;
use crate::core::{Core};
use crate::store::Store;

mod store;
mod entry;
mod core;
mod command;
mod executor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut repl: Core = Core::new();
    repl.run();
    Ok(())
}
