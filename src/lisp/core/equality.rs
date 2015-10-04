use lisp::types::{LispValue, LispResult, _true, _false};

pub fn equal_q(a: Vec<LispValue>) -> LispResult {
    if a[0] == a[1] {Ok(_true())} else {Ok(_false())}
}
