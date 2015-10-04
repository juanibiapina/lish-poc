#[derive(Debug)]
pub enum Error {
    Parser(String),
    BindingNotFound(String),
    ApplyInNonFunction,
    TypeError(String),
    SyntaxError(String),

    Message(String),
}
