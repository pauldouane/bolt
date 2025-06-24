use std::io::Error;
use crate::command::Command;
use crate::entry::Entry;
use crate::core::{ReplControl, ReplOutput};
use crate::store::Store;

pub struct Executor {}

impl Executor {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn execute_command(&self, command: &Command, store: &Store, output: &ReplOutput) -> Result<ReplControl, Error> {
        match command {
            Command::Get(key) => {
                if let Some(entry) = store.get(&key) {
                    output.repl_write_line(format!("\rValue: {:?}", entry))?;
                } else {
                    output.repl_write_line(format!("\rKey not found: {}", key))?;
                }
            }
            Command::Set(key, value, ttl) => {
                store.set(key.to_string(), Entry::new(value.to_string(), *ttl))
            }
            Command::Delete(key) => {
                store.delete(key);
            }
            Command::Keys => {
                store.keys(output);
            }
            Command::Exit => return Ok(ReplControl::Exit),
            Command::Empty => return Ok(ReplControl::Continue),
            Command::Unknown(cmd) => {
                output.repl_write_line(format!("\rUnknown command: {}", cmd))?;
            },
        }
        Ok(ReplControl::Continue)
    }
}