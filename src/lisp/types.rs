use std::rc::Rc;

use error::Error;
use lisp::printer;
use lisp::env::{Env, env_new, env_bind};

use self::LispType::*;

#[derive(Clone)]
pub enum LispType {
    Nil,
    True,
    False,
    Symbol(String),
    Strn(String),
    Int(isize),
    List(Vec<LispValue>),
    Vector(Vec<LispValue>),
    NativeFunction(fn(Vec<LispValue>) -> LispResult),
    Function(FunctionData),
}

pub type LispValue = Rc<LispType>;

pub type LispResult = Result<LispValue, Error>;

#[derive(Clone)]
pub struct FunctionData {
    pub eval:     fn(LispValue, Env) -> LispResult,
    pub exp:      LispValue,
    pub env:      Env,
    pub params:   LispValue,
    pub is_macro: bool,
}

impl LispType {
    pub fn apply(&self, args:Vec<LispValue>) -> LispResult {
        match *self {
            LispType::NativeFunction(f) => f(args),
            LispType::Function(ref data) => {
                let data = data.clone();
                let alst = list(args);
                let new_env = env_new(Some(data.env.clone()));
                match env_bind(&new_env, data.params, alst) {
                    Ok(_) => (data.eval)(data.exp, new_env),
                    Err(e) => Err(e),
                }
            },
            _ => Err(Error::ApplyInNonFunction),
        }

    }

    pub fn print(&self, print_readably: bool) -> String {
        match *self {
            LispType::Nil => "nil".to_string(),
            LispType::True => "true".to_string(),
            LispType::False => "false".to_string(),
            LispType::Int(v) => v.to_string(),
            LispType::Symbol(ref v) => v.clone(),
            LispType::Strn(ref v) => {
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
            LispType::Vector(ref v) => {
                pr_list(v, print_readably, "[", "]", " ")
            },
            LispType::NativeFunction(_) => {
                format!("#<native-function ...>")
            },
            LispType::Function(_) => {
                format!("(fn)")
            },
        }
    }
}

impl PartialEq for LispType {
    fn eq(&self, other: &LispType) -> bool {
        match (self, other) {
            (&Nil, &Nil) |
            (&True, &True) |
            (&False, &False) => true,
            (&Int(ref a), &Int(ref b)) => a == b,
            (&Strn(ref a), &Strn(ref b)) => a == b,
            (&Symbol(ref a), &Symbol(ref b)) => a == b,
            (&List(ref a), &List(ref b)) |
            (&Vector(ref a), &Vector(ref b)) |
            (&List(ref a), &Vector(ref b)) |
            (&Vector(ref a), &List(ref b)) => a == b,
            // TODO: fix these
            (&Function(_), &Function(_)) => false,
            (&NativeFunction(_), &NativeFunction(_)) => false,
            _ => return false,
        }
    }
}

fn pr_list(lst: &Vec<LispValue>, pr: bool, start: &str , end: &str, join: &str) -> String {
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

pub fn listv(seq:Vec<LispValue>) -> LispResult { Ok(list(seq)) }

pub fn list_q(a:Vec<LispValue>) -> LispResult {
    match *a[0].clone() {
        List(_) => Ok(_true()),
        _ => Ok(_false()),
    }
}

pub fn vector(seq: Vec<LispValue>) -> LispValue {
    Rc::new(LispType::Vector(seq))
}

pub fn vectorv(seq: Vec<LispValue>) -> LispResult { Ok(vector(seq)) }

pub fn vector_q(a:Vec<LispValue>) -> LispResult {
    match *a[0].clone() {
        Vector(_) => Ok(_true()),
        _           => Ok(_false()),
    }
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
    Rc::new(LispType::Strn(strn))
}

pub fn native_function(f: fn(Vec<LispValue>) -> LispResult) -> LispValue {
    Rc::new(LispType::NativeFunction(f))
}

pub fn function(eval: fn(LispValue, Env) -> LispResult, exp: LispValue, env: Env, params: LispValue) -> LispValue {
    Rc::new(LispType::Function(FunctionData{eval: eval, exp: exp, env: env, params: params, is_macro: false}))
}
