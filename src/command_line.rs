use std::process::Command;

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

    pub fn run(&self) {
        let output = Command::new(&self.command)
            .args(&self.args)
            .output().unwrap();

        println!("{}", String::from_utf8(output.stdout).unwrap().trim());
    }
}

