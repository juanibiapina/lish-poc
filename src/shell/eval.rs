use std::process::Command;

use shell::command_line::CommandLine;
use error::Error;

use lisp::types::string;
use lisp::env::{Env, env_get_export};

pub fn eval(command_line: CommandLine, env: Env) -> Result<(), Error> {
    if let Some(value) = env_get_export(&env.clone(), &command_line.command) {
        let mut args = vec!();

        for arg in command_line.args.iter() {
            args.push(string(arg.to_string()));
        };

        let result = try!(value.apply(args));

        println!("{}", result.print(false));

        Ok(())
    } else {
        let mut child = match Command::new(&command_line.command)
            .args(&command_line.args)
            .spawn() {
                Ok(child) => child,
                Err(_) => return Err(Error::CommandNotFound(command_line.command.to_string())),
            };

        child.wait().unwrap();

        Ok(())
    }
}
