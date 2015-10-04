use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use lisp::types::{LispValue, LispResult, _nil, list, symbol};
use lisp::types::LispType::{Symbol, List, Vector};

use error::Error;

struct EnvType {
    data: HashMap<String, LispValue>,
    exports: HashMap<String, LispValue>,
    outer: Option<Env>,
}

pub type Env = Rc<RefCell<EnvType>>;

pub fn env_new(outer: Option<Env>) -> Env {
    Rc::new(RefCell::new(EnvType{
        data: HashMap::new(),
        exports: HashMap::new(),
        outer: outer
    }))
}

pub fn env_export(env: &Env, name: &str) -> LispResult {
    let value = try!(env_get(env, &symbol(name)));

    env.borrow_mut().exports.insert(name.to_string(), value);

    Ok(_nil())
}

pub fn env_get_export(env: &Env, name: &str) -> Option<LispValue> {
    match env.borrow().exports.get(name) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

pub fn env_root(env: &Env) -> Env {
    match env.borrow().outer {
        Some(ref ei) => env_root(ei),
        None => env.clone(),
    }
}

pub fn env_bind(env: &Env,
                mbinds: LispValue,
                mexprs: LispValue) -> Result<Env, Error> {
    let mut variadic = false;
    match *mbinds {
        List(ref binds) | Vector(ref binds) => {
            match *mexprs {
                List(ref exprs) | Vector(ref exprs) => {
                    let mut it = binds.iter().enumerate();
                    for (i, b) in it.by_ref() {
                        match **b {
                            Symbol(ref strn) => {
                                if *strn == "&" {
                                    variadic = true;
                                    break;
                                } else {
                                    env_set(env, b.clone(), exprs[i].clone());
                                }
                            }
                            _ => return Err(Error::Message("non-symbol bind".to_string())),
                        }
                    }
                    if variadic {
                        let (i, sym) = it.next().unwrap();
                        match **sym {
                            Symbol(_) => {
                                let rest = exprs[i-1..].to_vec();
                                env_set(env, sym.clone(), list(rest));
                            }
                            _ => return Err(Error::Message("& bind to non-symbol".to_string())),
                        }
                    }
                    Ok(env.clone())
                },
                _ => Err(Error::Message("exprs must be a list".to_string())),
            }
        },
        _ => Err(Error::Message("binds must be a list".to_string())),
    }
}

pub fn env_find(env: &Env, key: &LispValue) -> Option<Env> {
    match **key {
        Symbol(ref k) => {
            let map = env.borrow();
            if map.data.contains_key(k) {
                Some(env.clone())
            } else {
                match map.outer {
                    Some(ref e) => env_find(e, key),
                    None => None,
                }
            }
        },
        _ => None
    }
}

pub fn env_set(env: &Env, key: LispValue, val: LispValue) {
    match *key {
        Symbol(ref k) => { env.borrow_mut().data.insert(k.to_string(), val); }
        _ => panic!("unreachable code"),
    }
}

pub fn env_get(env: &Env, key: &LispValue) -> LispResult {
    match **key {
        Symbol(ref k) => {
            match env_find(env, key) {
                Some(e) => {
                    match e.borrow().data.get(k) {
                        Some(v) => Ok(v.clone()),
                        None => Ok(_nil()),
                    }
                },
                None => Err(Error::BindingNotFound(k.to_string())),
            }
        }
        _ => panic!("env_get called with non-symbol key"),
    }
}
