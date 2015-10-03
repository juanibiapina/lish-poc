use lisp::error::Error as LispError;
use shell::error::Error as ShellError;

pub enum Error {
    Comment,
    EndOfInput,
    EmptyInput,

    Lisp(LispError),
    Shell(ShellError),
}

impl From<LispError> for Error {
    fn from(err: LispError) -> Error {
        Error::Lisp(err)
    }
}

impl From<ShellError> for Error {
    fn from(err: ShellError) -> Error {
        Error::Shell(err)
    }
}
