use crate::entry::Entry;

pub enum Command {
    Get(String),
    Set(String, String, Option<u64>),
    Keys,
    Delete(String),
    Exit,
    Empty,
    Unknown(String),
}

impl Command {
    pub fn parse(input: String) -> Command {
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();
        match tokens.as_slice() {
            ["get", key] => Command::Get(key.to_string()),
            ["set", key, value] => {
                Command::Set(key.to_string(), value.to_string(), None)
            },
            ["set", key, value, ttl] => {
                Command::Set(key.to_string(), value.to_string(), Some(ttl.parse::<u64>().unwrap()))
            }
            ["keys"] => Command::Keys,
            ["delete", key] => Command::Delete(key.to_string()),
            ["exit"] => Command::Exit,
            [] => Command::Empty,
            _ => Command::Unknown(input.trim().to_string()),
        }
    }
}