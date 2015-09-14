use std::rc::Rc;

use error::Error;
use lisp::printer;

pub enum LispType {
    Nil,
    True,
    False,
    Symbol(String),
    String(String),
    Int(isize),
    List(Vec<LispValue>),
}

pub type LispValue = Rc<LispType>;

pub type LispResult = Result<LispValue, Error>;

impl LispType {
    pub fn print(&self, print_readably: bool) -> String {
        match *self {
            LispType::Nil => "nil".to_string(),
            LispType::True => "true".to_string(),
            LispType::False => "false".to_string(),
            LispType::Int(v) => v.to_string(),
            LispType::Symbol(ref v) => v.clone(),
            LispType::String(ref v) => {
                if v.starts_with("\u{29e}") {
                    format!(":{}", &v[2..])
                } else if print_readably {
                    printer::escape_str(v)
                } else {
                    v.clone()
                }
            },
            LispType::List(ref v) => {
                pr_list(v, print_readably, "(", ")", " ")
            },
        }
    }
}

pub fn pr_list(lst: &Vec<LispValue>, pr: bool, start: &str , end: &str, join: &str) -> String {
    let mut first = true;
    let mut res = String::new();
    res.push_str(start);
    for mv in lst.iter() {
        if first {
            first = false;
        } else {
            res.push_str(join);
        }
        res.push_str(&mv.print(pr));
    }
    res.push_str(end);
    res
}

// Constructors
pub fn list(seq: Vec<LispValue>) -> LispValue {
    Rc::new(LispType::List(seq))
}

pub fn _int(i: isize) -> LispValue {
    Rc::new(LispType::Int(i))
}

pub fn _nil() -> LispValue {
    Rc::new(LispType::Nil)
}

pub fn _true() -> LispValue {
    Rc::new(LispType::True)
}

pub fn _false() -> LispValue {
    Rc::new(LispType::False)
}

pub fn symbol(strn: &str) -> LispValue {
    Rc::new(LispType::Symbol(strn.to_string()))
}

pub fn string(strn: String) -> LispValue {
    Rc::new(LispType::String(strn))
}