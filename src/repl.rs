use readline;
use error::Error;

use shell;
use shell::error::Error as ShellError;

use lisp;
use lisp::env::Env;
use lisp::error::Error as LispError;
use lisp::core;

pub struct Repl {
    env: Env,
}

impl Repl {
    pub fn new() -> Repl {
        Repl{
            env: core::env::create(),
        }
    }

    pub fn run(&self) {
        loop {
            match self.rep() {
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

    fn rep(&self) -> Result<(), Error> {
        let input = match readline::lish_readline(":) ") {
            Some(input) => input,
            None => return Err(Error::EndOfInput),
        };

        let trimmed_input = input.trim();

        if trimmed_input.len() == 0 {
            return Err(Error::EmptyInput);
        }

        self.process(trimmed_input.to_string())
    }

    fn process(&self, input: String) -> Result<(), Error> {
        if input.starts_with("#") {
            return Err(Error::Comment);
        }

        if input.starts_with("(") || input.starts_with("'") || input.starts_with("`") || input.starts_with("~") {
            self.process_lisp(input)
        } else {
            self.process_shell(input)
        }
    }

    fn process_lisp(&self, input: String) -> Result<(), Error> {
        let mut reader = lisp::reader::Reader::new(input);

        // read
        let ast = try!(reader.read_form());

        // eval
        let result = try!(lisp::eval::eval(ast, self.env.clone()));

        // print
        println!("{}", result.print(true));

        Ok(())
    }

    fn process_shell(&self, input: String) -> Result<(), Error> {
        let mut reader = shell::reader::Reader::new(input);

        // read
        let command_line = try!(reader.read_command());

        // eval
        try!(shell::eval::eval(command_line, self.env.clone()));

        Ok(())
    }
}
