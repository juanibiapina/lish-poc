use std::process::Command;

use shell::command_line::CommandLine;
use shell::error::Error;

pub fn eval(command_line: CommandLine) -> Result<(), Error> {
    let mut child = match Command::new(&command_line.command)
        .args(&command_line.args)
        .spawn() {
            Ok(child) => child,
            Err(_) => return Err(Error::CommandNotFound(command_line.command.to_string())),
        };

    child.wait().unwrap();

    Ok(())
}
