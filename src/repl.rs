use readline;
use error::Error;

use shell;
use shell::error::Error as ShellError;

use lisp;
use lisp::error::Error as LispError;
use lisp::types;
use lisp::eval;
use lisp::env::{Env, env_new, env_set};
use lisp::core;

pub fn run() {
    let repl_env = env_new(None);
    for (k, v) in core::ns().into_iter() {
        env_set(&repl_env, types::symbol(k), v);
    }

    loop {
        match rep(repl_env.clone()) {
            Ok(()) => {},
            Err(Error::Comment) => {
                continue;
            },
            Err(Error::EndOfInput) => {
                println!("");
                break;
            },
            Err(Error::EmptyInput) => {
                continue;
            },
            Err(Error::Lisp(err)) => {
                match err {
                    LispError::Parser(message) => {
                        println!("{}", message);
                    },
                    LispError::BindingNotFound(name) => {
                        println!("{} not found", name);
                    },
                    LispError::ApplyInNonFunction => {
                        println!("trying to apply a non function");
                    },
                    LispError::TypeError(message) => {
                        println!("{}", message);
                    },
                    LispError::Message(message) => {
                        println!("{}", message);
                    },
                }
            },
            Err(Error::Shell(err)) => {
                match err {
                    ShellError::CommandNotFound(command) => {
                        println!("lish: command not found: {}", command);
                    },
                }
            },
        };
    }
}

fn rep(env: Env) -> Result<(), Error> {
    let input = match readline::lish_readline(":) ") {
        Some(input) => input,
        None => return Err(Error::EndOfInput),
    };

    let trimmed_input = input.trim();

    if trimmed_input.len() == 0 {
        return Err(Error::EmptyInput);
    }

    process(trimmed_input.to_string(), env)
}

fn process(input: String, env: Env) -> Result<(), Error> {
    if input.starts_with("#") {
        return Err(Error::Comment);
    }

    if input.starts_with("(") || input.starts_with("'") || input.starts_with("`") || input.starts_with("~") {
        process_lisp(input, env)
    } else {
        process_shell(input)
    }
}

fn process_lisp(input: String, env: Env) -> Result<(), Error> {
    let mut reader = lisp::reader::Reader::new(input);

    // read
    let ast = try!(reader.read_form());

    // eval
    let result = try!(eval::eval(ast, env.clone()));

    // print
    println!("{}", result.print(true));

    Ok(())
}

fn process_shell(input: String) -> Result<(), Error> {
    let mut reader = shell::reader::Reader::new(input);

    let command_line = try!(reader.read_command());

    try!(command_line.run());

    Ok(())
}
