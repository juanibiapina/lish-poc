use lisp::types::{LispType, LispValue, LispResult, _true, _false, list};

pub fn listv(seq:Vec<LispValue>) -> LispResult { Ok(list(seq)) }

pub fn list_q(a:Vec<LispValue>) -> LispResult {
    match *a[0].clone() {
        LispType::List(_) => Ok(_true()),
        _ => Ok(_false()),
    }
}

