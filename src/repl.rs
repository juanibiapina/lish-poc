use readline;
use error::Error;
use shell::command_line::CommandLine;
use lisp;
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
            Err(Error::EmptyCommand) => {
                continue;
            },
            Err(Error::CommandNotFound(command)) => {
                println!("lish: command not found: {}", command);
            },
            Err(Error::Parser(message)) => {
                println!("{}", message);
            },
            Err(Error::BindingNotFound(name)) => {
                println!("{} not found", name);
            },
            Err(Error::ApplyInNonFunction) => {
                println!("trying to apply a non function");
            },
            Err(Error::TypeError(message)) => {
                println!("{}", message);
            },
            Err(Error::Message(message)) => {
                println!("{}", message);
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
        return Err(Error::EmptyCommand);
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
    let command_line = CommandLine::parse(input);

    command_line.run()
}
