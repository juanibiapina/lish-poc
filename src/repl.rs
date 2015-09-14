use readline;

use error::Error;
use shell::command_line::CommandLine;
use lisp;

fn process_lisp(input: String) -> Result<(), Error> {
    let mut reader = lisp::reader::Reader::new(input);

    let result = try!(reader.read_form());

    println!("{}", result.print());

    Ok(())
}

fn process_shell(input: String) -> Result<(), Error> {
    let command_line = CommandLine::parse(input);

    try!(command_line.run());

    Ok(())
}

fn process(input: String) -> Result<(), Error> {
    if input.starts_with("(") {
        process_lisp(input)
    } else {
        process_shell(input)
    }
}

fn rep() -> Result<(), Error> {
    let input = match readline::lish_readline(":) ") {
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
                println!("lish: command not found: {}", command);
            },
            Err(Error::Parser(message)) => {
                println!("{}", message);
            },
        };
    }
}
