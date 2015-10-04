use lisp::error::Error;
use lisp::reader;
use lisp::types::{LispValue, LispResult, LispType};

pub fn read(a: Vec<LispValue>) -> LispResult {
    match *a[0] {
        LispType::Strn(ref a0) => reader::read(a0),
        _ => Err(Error::TypeError("argument must be a string".to_string())),
    }
}
