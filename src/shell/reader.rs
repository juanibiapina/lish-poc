extern crate regex;

macro_rules! regex {
    ($e:expr) => (regex::Regex::new($e).unwrap())
}

use shell::command_line::CommandLine;

pub struct Reader {
    tokens: Vec<String>,
}

impl Reader {
    pub fn new(str: String) -> Reader {
        Reader{
            tokens: tokenize(str),
        }
    }

    pub fn read_command(&mut self) -> CommandLine {
        let mut iter = self.tokens.iter();

        let command = iter.next().unwrap().to_string();
        let mut args = vec!();
        for part in iter {
            args.push(part.to_string());
        }

        CommandLine {
            command: command,
            args: args,
        }
    }
}

fn tokenize(string: String) -> Vec<String> {
    let parts = string.split(" ");

    let mut result = vec!();
    for part in parts {
        result.push(part.to_string());
    }

    result
}
