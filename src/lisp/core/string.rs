use lisp::types::{LispValue, LispResult, string, _nil};
use lisp::printer;

pub fn str(a: Vec<LispValue>) -> LispResult {
    Ok(string(printer::pr_list(&a, false, "", "", "")))
}

pub fn println(a: Vec<LispValue>) -> LispResult {
    println!("{}", printer::pr_list(&a, false, "", "", " "));
    Ok(_nil())
}
