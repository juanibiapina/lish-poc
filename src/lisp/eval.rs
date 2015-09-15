use std::collections::HashMap;

use lisp::types;
use error::Error;

pub fn eval(ast: types::LispValue, env: &HashMap<String, types::LispValue>) -> types::LispResult {
    match *ast {
        types::LispType::List(_) => {
        },
        _ => {
            return eval_ast(ast, env)
        },
    }

    // apply list
    match *try!(eval_ast(ast, env)) {
        types::LispType::List(ref args) => {
            let ref f = args.clone()[0];
            f.apply(args[1..].to_vec())
        },
        _ => panic!("unreachable code"),
    }
}

fn eval_ast(ast: types::LispValue, env: &HashMap<String, types::LispValue>) -> types::LispResult {
    match *ast {
        types::LispType::Symbol(ref sym) => {
            match env.get(sym) {
                Some(mv) => Ok(mv.clone()),
                None     => Err(Error::BindingNotFound(sym.to_string())),
            }
        },
        types::LispType::List(ref a) | types::LispType::Vector(ref a) => {
            let mut ast_vec : Vec<types::LispValue> = vec![];
            for mv in a.iter() {
                let mv2 = mv.clone();
                ast_vec.push(try!(eval(mv2, env)));
            }
            Ok(match *ast { types::LispType::List(_) => types::list(ast_vec),
                            _                          => types::vector(ast_vec) })
        },
        _ => Ok(ast.clone()),
    }
}
