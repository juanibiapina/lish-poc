use readline;
use error::Error;
use shell::command_line::CommandLine;
use lisp;
use lisp::types;
use lisp::eval;
use lisp::env::{Env, env_new, env_set};

fn int_op<F>(f: F, a:Vec<types::LispValue>) -> types::LispResult
    where F: FnOnce(isize, isize) -> isize
{
    match *a[0] {
        types::LispType::Int(a0) => match *a[1] {
            types::LispType::Int(a1) => Ok(types::_int(f(a0,a1))),
            _ => Err(Error::TypeError("second arg must be an int".to_string())),
        },
        _ => Err(Error::TypeError("first arg must be an int".to_string())),
    }
}

fn add(a:Vec<types::LispValue>) -> types::LispResult { int_op(|i,j| { i+j }, a) }
fn sub(a:Vec<types::LispValue>) -> types::LispResult { int_op(|i,j| { i-j }, a) }
fn mul(a:Vec<types::LispValue>) -> types::LispResult { int_op(|i,j| { i*j }, a) }
fn div(a:Vec<types::LispValue>) -> types::LispResult { int_op(|i,j| { i/j }, a) }

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

fn process_shell(input: String, env: Env) -> Result<(), Error> {
    let command_line = CommandLine::parse(input);

    try!(command_line.run());

    Ok(())
}

fn process(input: String, env: Env) -> Result<(), Error> {
    if input.starts_with("#") {
        return Err(Error::Comment);
    }

    if input.starts_with("(") || input.starts_with("'") || input.starts_with("`") || input.starts_with("~") {
        process_lisp(input, env)
    } else {
        process_shell(input, env)
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

    try!(process(trimmed_input.to_string(), env));

    Ok(())
}

pub fn run() {
    // repl env
    let repl_env = env_new(None);
    env_set(&repl_env, types::symbol("+"), types::function(add));
    env_set(&repl_env, types::symbol("-"), types::function(sub));
    env_set(&repl_env, types::symbol("*"), types::function(mul));
    env_set(&repl_env, types::symbol("/"), types::function(div));

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
        };
    }
}
