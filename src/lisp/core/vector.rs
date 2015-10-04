use lisp::types::{LispType, LispValue, LispResult, _true, _false, vector};

pub fn vectorv(seq: Vec<LispValue>) -> LispResult { Ok(vector(seq)) }

pub fn vector_q(a:Vec<LispValue>) -> LispResult {
    match *a[0].clone() {
        LispType::Vector(_) => Ok(_true()),
        _ => Ok(_false()),
    }
}

