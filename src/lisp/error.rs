pub enum Error {
    Parser(String),
    BindingNotFound(String),
    ApplyInNonFunction,
    TypeError(String),

    Message(String),
}
