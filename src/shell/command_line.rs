pub struct CommandLine {
    pub command: String,
    pub args: Vec<String>,
}

impl CommandLine {
    pub fn replace_alias(&self, name: String) -> CommandLine {
        CommandLine{
            command: name,
            args: self.args.clone(),
        }
    }
}
