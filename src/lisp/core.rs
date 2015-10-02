use std::collections::HashMap;

use lisp::types::{self, LispType, LispValue, LispResult, _true, _false, _int,
                  native_function};
use error::Error;

fn equal_q(a: Vec<LispValue>) -> LispResult {
    if a[0] == a[1] {Ok(_true())} else {Ok(_false())}
}

fn int_op<F>(f: F, a:Vec<LispValue>) -> LispResult
    where F: FnOnce(isize, isize) -> isize
{
    match *a[0] {
        LispType::Int(a0) => match *a[1] {
            LispType::Int(a1) => Ok(_int(f(a0,a1))),
            _ => Err(Error::TypeError("second arg must be an int".to_string())),
        },
        _ => Err(Error::TypeError("first arg must be an int".to_string())),
    }
}

fn bool_op<F>(f: F, a: Vec<LispValue>) -> LispResult
    where F: FnOnce(isize, isize) -> bool
{
    match *a[0] {
        LispType::Int(a0) => match *a[1] {
            LispType::Int(a1) => {
                match f(a0,a1) {
                    true => Ok(_true()),
                    false => Ok(_false()),
                }
            },
            _ => Err(Error::TypeError("second arg must be an int".to_string())),
        },
        _ => Err(Error::TypeError("first arg must be an int".to_string())),
    }
}

fn add(a:Vec<LispValue>) -> LispResult { int_op(|i,j| { i+j }, a) }
fn sub(a:Vec<LispValue>) -> LispResult { int_op(|i,j| { i-j }, a) }
fn mul(a:Vec<LispValue>) -> LispResult { int_op(|i,j| { i*j }, a) }
fn div(a:Vec<LispValue>) -> LispResult { int_op(|i,j| { i/j }, a) }

pub fn lt (a: Vec<LispValue>) -> LispResult { bool_op(|i,j| { i<j }, a) }
pub fn lte(a: Vec<LispValue>) -> LispResult { bool_op(|i,j| { i<=j }, a) }
pub fn gt (a: Vec<LispValue>) -> LispResult { bool_op(|i,j| { i>j }, a) }
pub fn gte(a: Vec<LispValue>) -> LispResult { bool_op(|i,j| { i>=j }, a) }

pub fn ns() -> HashMap<&'static str, LispValue> {
    let mut ns = HashMap::new();;

    ns.insert("=", native_function(equal_q));

    ns.insert("+", native_function(add));
    ns.insert("-", native_function(sub));
    ns.insert("*", native_function(mul));
    ns.insert("/", native_function(div));
    ns.insert("<",  native_function(lt));
    ns.insert("<=", native_function(lte));
    ns.insert(">",  native_function(gt));
    ns.insert(">=", native_function(gte));

    ns.insert("list", native_function(types::listv));
    ns.insert("list?", native_function(types::list_q));
    ns.insert("vector", native_function(types::vectorv));
    ns.insert("vector?", native_function(types::vector_q));

    ns
}
