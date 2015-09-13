use std::error::Error as StdError;

pub enum Error {
    CommandNotFound(String),
}
