use readline;

use error::Error;
use command_line::CommandLine;

fn process(input: String) -> Result<(), Error> {
    let command_line = CommandLine::parse(input);

    try!(command_line.run());

    Ok(())
}

fn rep() -> Result<(), Error> {
    let input = match readline::readline(":) ") {
        Some(input) => input,
        None => return Err(Error::EndOfInput),
    };

    let trimmed_input = input.trim();

    if trimmed_input.len() == 0 {
        return Err(Error::EmptyCommand);
    }

    try!(process(trimmed_input.to_string()));

    Ok(())
}

pub fn run() {
    loop {
        match rep() {
            Ok(()) => {},
            Err(Error::EndOfInput) => {
                println!("");
                break;
            },
            Err(Error::EmptyCommand) => {
                continue;
            },
            Err(Error::CommandNotFound(command)) => {
                println!("lish: command not found: {}", command)
            },
        };
    }
}
