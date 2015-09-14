pub enum Error {
    EndOfInput,
    EmptyCommand,
    CommandNotFound(String),
    Parser(String),
}
