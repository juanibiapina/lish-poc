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
        let mut child = match Command::new(&self.command)
            .args(&self.args)
            .spawn() {
                Ok(child) => child,
                Err(e) => {
                    println!("lish: command not found: {}", &self.command);
                    return;
                },
            };

        child.wait().unwrap();
    }
}

