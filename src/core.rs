use std::fmt::format;
use std::io;
use std::io::{stdin, stdout, Write};
use crate::command::Command;
use crate::executor::Executor;
use crate::store::Store;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType};

pub(crate) struct Core {
    store: Store,
    output: ReplOutput,
    buffer: String,
    history: Vec<String>,
    history_index: Option<usize>,
}

pub(crate) enum ReplControl {
    Continue,
    Exit
}

impl Core {
    pub fn new() -> Self {
        Self {
            store: Store::new(),
            output: ReplOutput::new(),
            buffer: String::new(),
            history: Vec::new(),
            history_index: None,
        }
    }

    pub fn run(&mut self) {
        enable_raw_mode().unwrap();

        loop {
            execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
            print!("\rbolt> {}", self.buffer);
            stdout().flush().unwrap();

            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char(c) => {
                        self.buffer.push(c);
                    }
                    KeyCode::Enter => {
                        self.output.repl_write_line("").unwrap();
                        self.history_index = None;
                        let command: Command = Command::parse(self.buffer.clone());

                        let executor: Executor = Executor::new();

                        match executor.execute_command(&command, &mut self.store, &self.output) {
                            Ok(ReplControl::Exit) => {
                                self.output.repl_write("\rBye!").unwrap();
                                self.output.repl_write_line("\r").unwrap();
                                break;
                            },
                            Ok(ReplControl::Continue) => {
                                if !self.buffer.is_empty() {
                                    self.history.push(self.buffer.clone());
                                }
                                self.buffer.clear();
                                continue
                            },
                            Err(e) => panic!("{}", e),
                        }
                    },
                    KeyCode::Up => {
                        if let Some(i) = self.history_index {
                            if i > 0 {
                                self.history_index = Some(i - 1);
                            }
                        } else if !self.history.is_empty() {
                            self.history_index = Some(self.history.len() - 1);
                        }

                        if let Some(i) = self.history_index {
                            self.buffer = self.history[i].clone();
                        }
                    }
                    KeyCode::Down => {
                        if let Some(i) = self.history_index {
                            if i + 1 < self.history.len() {
                                self.history_index = Some(i + 1);
                                self.buffer = self.history[self.history_index.unwrap()].clone();
                            } else {
                                self.history_index = None;
                                self.buffer.clear();
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        self.buffer.pop();
                        if self.buffer.is_empty() {
                            self.history_index = None;
                        }
                    }
                    _ => {}
                }
            }
        }
        disable_raw_mode().unwrap();
        self.store.save_to_file().unwrap()
    }
}

pub(crate) struct ReplOutput {}

impl ReplOutput {
    pub fn new() -> Self {
        ReplOutput {}
    }

    pub fn repl_write_line<S: AsRef<str>>(&self, text: S) -> io::Result<()> {
        let mut stdout = stdout();
        stdout.write_all(text.as_ref().as_bytes())?;
        stdout.write_all(b"\n")?;
        stdout.flush()?;
        Ok(())
    }

    pub fn repl_write<S: AsRef<str>>(&self, text: S) -> io::Result<()> {
        let mut stdout = stdout();
        stdout.write_all(text.as_ref().as_bytes())?;
        stdout.flush()?;
        Ok(())
    }
}