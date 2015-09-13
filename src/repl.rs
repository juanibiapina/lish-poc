use readline;

use error::Error;
use command_line::CommandLine;

fn process(input: String) {
    let command_line = CommandLine::parse(input);

    match command_line.run() {
        Ok(()) => {},
        Err(Error::CommandNotFound(command)) => {
            println!("lish: command not found: {}", command)
        },
    };
}

pub fn run() {
    loop {
        let input = match readline::readline(":) ") {
            Some(input) => input,
            None => {
                println!("");
                break;
            },
        };

        let trimmed_input = input.trim();

        if trimmed_input.len() == 0 {
            continue
        }

        process(trimmed_input.to_string());
    }
}
