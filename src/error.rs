#[derive(Debug)]
pub enum Error {
    Comment,
    EndOfInput,
    EmptyInput,

    // lisp errors
    Parser(String),
    BindingNotFound(String),
    ApplyInNonFunction,
    TypeError(String),
    Message(String),

    // shell errors
    CommandNotFound(String),
}
