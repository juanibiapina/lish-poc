pub enum Error {
    Comment,
    EndOfInput,
    EmptyCommand,
    CommandNotFound(String),
    Parser(String),
}
