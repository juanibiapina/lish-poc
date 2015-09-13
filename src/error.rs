use std::error::Error as StdError;

pub enum Error {
    EndOfInput,
    EmptyCommand,
    CommandNotFound(String),
}
