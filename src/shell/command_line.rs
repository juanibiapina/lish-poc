use std::process::Command;

use shell::error::Error;

pub struct CommandLine {
    command: String,
    args: Vec<String>,
}

impl CommandLine {
    pub fn parse(string: String) -> CommandLine {
        let mut parts = string.split(" ");

        let command = parts.next().unwrap().to_string();

        let mut args = vec!();
        for arg in parts {
            args.push(arg.to_string());
        }

        CommandLine {
            command: command,
            args: args,
        }
    }

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

