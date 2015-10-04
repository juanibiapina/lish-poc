use std::collections::HashMap;

use lisp::types::{symbol, LispValue, native_function};
use lisp::env::{Env, env_new, env_set};

use lisp::core::equality;
use lisp::core::list;
use lisp::core::vector;
use lisp::core::math;

fn ns() -> HashMap<&'static str, LispValue> {
    let mut ns = HashMap::new();;

    ns.insert("=", native_function(equality::equal_q));

    ns.insert("+", native_function(math::add));
    ns.insert("-", native_function(math::sub));
    ns.insert("*", native_function(math::mul));
    ns.insert("/", native_function(math::div));
    ns.insert("<",  native_function(math::lt));
    ns.insert("<=", native_function(math::lte));
    ns.insert(">",  native_function(math::gt));
    ns.insert(">=", native_function(math::gte));

    ns.insert("list", native_function(list::listv));
    ns.insert("list?", native_function(list::list_q));
    ns.insert("vector", native_function(vector::vectorv));
    ns.insert("vector?", native_function(vector::vector_q));

    ns
}

pub fn env() -> Env {
    let env = env_new(None);

    for (k, v) in ns().into_iter() {
        env_set(&env, symbol(k), v);
    }

    env
}
