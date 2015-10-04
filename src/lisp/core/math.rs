use error::Error;
use lisp::types::{LispType, LispValue, LispResult, _true, _false, _int};

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

pub fn add(a:Vec<LispValue>) -> LispResult { int_op(|i,j| { i+j }, a) }
pub fn sub(a:Vec<LispValue>) -> LispResult { int_op(|i,j| { i-j }, a) }
pub fn mul(a:Vec<LispValue>) -> LispResult { int_op(|i,j| { i*j }, a) }
pub fn div(a:Vec<LispValue>) -> LispResult { int_op(|i,j| { i/j }, a) }

pub fn lt (a: Vec<LispValue>) -> LispResult { bool_op(|i,j| { i<j }, a) }
pub fn lte(a: Vec<LispValue>) -> LispResult { bool_op(|i,j| { i<=j }, a) }
pub fn gt (a: Vec<LispValue>) -> LispResult { bool_op(|i,j| { i>j }, a) }
pub fn gte(a: Vec<LispValue>) -> LispResult { bool_op(|i,j| { i>=j }, a) }

