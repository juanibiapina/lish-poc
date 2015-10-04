use lisp::reader::Reader;
use lisp::env::Env;
use lisp::error::Error;
use lisp::eval::eval;
use lisp::core;

pub struct Engine {
    pub env: Env,
}

impl Engine {
    pub fn new() -> Engine {
        Engine{
            env: core::env::create(),
        }
    }

    pub fn run(&self, input: String) -> Result<String, Error> {
        let mut reader = Reader::new(input);

        // read
        let ast = try!(reader.read_form());

        // eval
        let result = try!(eval(ast, self.env.clone()));

        // print
        Ok(result.print(true))
    }
}
