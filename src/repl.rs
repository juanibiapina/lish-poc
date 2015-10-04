use readline;
use error::Error;
use config::Config;

use shell;
use shell::error::Error as ShellError;

use lisp::engine::Engine as LispEngine;
use lisp::error::Error as LispError;

pub struct Repl {
    lisp_engine: LispEngine,
}

impl Repl {
    pub fn new(config: Config) -> Repl {
        let lisp_engine = LispEngine::new();

        let load_init_command = format!("(load-file! \"{}\")", &config.init_file);

        lisp_engine.run(&load_init_command);

        Repl{
            lisp_engine: lisp_engine,
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
        let result = try!(self.lisp_engine.run(&input));

        println!("{}", result);

        Ok(())
    }

    fn process_shell(&self, input: String) -> Result<(), Error> {
        let mut reader = shell::reader::Reader::new(input);

        // read
        let command_line = try!(reader.read_command());

        // eval
        try!(shell::eval::eval(command_line, self.lisp_engine.env.clone()));

        Ok(())
    }
}
