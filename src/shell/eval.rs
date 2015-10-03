use std::process::Command;

use shell::command_line::CommandLine;
use shell::error::Error;

use lisp::env::{Env, env_get_alias};

pub fn eval(command_line: CommandLine, env: Env) -> Result<(), Error> {
    let resolved;

    if let Some(name) = env_get_alias(&env, &command_line.command) {
        resolved = command_line.replace_alias(name);
    } else {
        resolved = command_line;
    }

    let mut child = match Command::new(&resolved.command)
        .args(&resolved.args)
        .spawn() {
            Ok(child) => child,
            Err(_) => return Err(Error::CommandNotFound(resolved.command.to_string())),
        };

    child.wait().unwrap();

    Ok(())
}
