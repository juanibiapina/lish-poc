use lisp::types;
use lisp::env::{Env, env_new, env_get, env_set};

enum FormType {
    Def,
    Let,
    Function,
}

impl FormType {
    pub fn from(name: &str) -> FormType {
        match name {
            "def!" => FormType::Def,
            "let" => FormType::Let,
            _ => FormType::Function,
        }
    }
}

pub fn eval(ast: types::LispValue, env: Env) -> types::LispResult {
    match *ast {
        types::LispType::List(_) => return eval_list(ast, env),
        _ => return eval_ast(ast, env),
    };
}

fn eval_list(ast: types::LispValue, env: Env) -> types::LispResult {
    let (form_type, elements) = match *ast {
        types::LispType::List(ref elements) => {
            let ref head = *elements[0];
            match *head {
                types::LispType::Symbol(ref name) => (FormType::from(name), elements),
                _ => (FormType::Function, elements),
            }
        },
        _ => panic!("unreachable code"),
    };

    match form_type {
        FormType::Def => eval_def(elements, env),
        FormType::Let => eval_let(elements, env),
        FormType::Function => eval_function(ast.clone(), env),
    }
}

fn eval_ast(ast: types::LispValue, env: Env) -> types::LispResult {
    match *ast {
        types::LispType::Symbol(_) => env_get(&env, &ast),
        types::LispType::List(ref a) | types::LispType::Vector(ref a) => {
            let mut ast_vec : Vec<types::LispValue> = vec![];
            for mv in a.iter() {
                let mv2 = mv.clone();
                ast_vec.push(try!(eval(mv2, env.clone())));
            }
            Ok(match *ast { types::LispType::List(_) => types::list(ast_vec),
            _                          => types::vector(ast_vec) })
        },
        _ => Ok(ast.clone()),
    }
}

fn eval_def(elements: &Vec<types::LispValue>, env: Env) -> types::LispResult {
    let a1 = (*elements)[1].clone();
    let a2 = (*elements)[2].clone();
    let r = try!(eval(a2, env.clone()));
    match *a1 {
        types::LispType::Symbol(_) => {
            env_set(&env.clone(), a1, r.clone());
            return Ok(types::_nil());
        },
        _ => panic!("def! of non-symbol"),
    }
}

fn eval_let(elements: &Vec<types::LispValue>, env: Env) -> types::LispResult {
    let let_env = env_new(Some(env.clone()));
    let a1 = (*elements)[1].clone();
    let a2 = (*elements)[2].clone();
    match *a1 {
        types::LispType::List(ref binds) | types::LispType::Vector(ref binds) => {
            let mut it = binds.iter();
            while it.len() >= 2 {
                let b = it.next().unwrap();
                let exp = it.next().unwrap();
                match **b {
                    types::LispType::Symbol(_) => {
                        let r = try!(eval(exp.clone(), let_env.clone()));
                        env_set(&let_env, b.clone(), r);
                    },
                    _ => panic!("let* with non-symbol binding"),
                }
            }
        },
        _ => panic!("let* with non-list bindings"),
    }
    return eval(a2, let_env.clone());
}

fn eval_function(ast: types::LispValue, env: Env) -> types::LispResult {
    let el = try!(eval_ast(ast.clone(), env.clone()));
    let args = match *el {
        types::LispType::List(ref args) => args,
        _ => panic!("unreachable code"),
    };
    let ref f = args.clone()[0];
    f.apply(args[1..].to_vec())
}
