use lisp::reader;
use lisp::env::Env;
use error::Error;
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
        engine.run("(def! load-file! (fn (f) (eval (read (str \"(do \" (slurp f) \")\")))))").unwrap();

        engine
    }

    pub fn run(&self, input: &str) -> Result<String, Error> {
        // read
        let ast = try!(reader::read(input));

        // eval
        let result = try!(eval(ast, self.env.clone()));

        // print
        Ok(result.print(true))
    }
}
