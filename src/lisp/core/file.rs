use std::fs::File;
use std::io::Read;

use error::Error;
use lisp::types::{LispValue, LispResult, LispType, string};

pub fn slurp(a: Vec<LispValue>) -> LispResult {
    match *a[0] {
        LispType::Strn(ref a0) => {
            let mut s = String::new();
            match File::open(a0).and_then(|mut f| f.read_to_string(&mut s)) {
                Ok(_) => Ok(string(s)),
                Err(e) => Err(Error::Message(e.to_string())),
            }
        },
        _ => Err(Error::TypeError("argument must be a string".to_string())),
    }
}
