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
        let core_env = core::env::create();

        let engine = Engine{
            env: core_env,
        };

        engine.run("(def! not (fn (a) (if a false true)))").unwrap();

        engine
    }

    pub fn run(&self, input: &str) -> Result<String, Error> {
        let mut reader = Reader::new(input.to_string());

        // read
        let ast = try!(reader.read_form());

        // eval
        let result = try!(eval(ast, self.env.clone()));

        // print
        Ok(result.print(true))
    }
}
