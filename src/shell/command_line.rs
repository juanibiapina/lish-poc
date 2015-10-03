use std::process::Command;

use shell::error::Error;

pub struct CommandLine {
    pub command: String,
    pub args: Vec<String>,
}

impl CommandLine {
    pub fn run(&self) -> Result<(), Error> {
        let mut child = match Command::new(&self.command)
            .args(&self.args)
            .spawn() {
                Ok(child) => child,
                Err(_) => return Err(Error::CommandNotFound(self.command.to_string())),
            };

        child.wait().unwrap();

        Ok(())
    }
}

