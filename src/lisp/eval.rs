use lisp::types::{LispValue, LispType, LispResult, list, vector, _nil, function};
use lisp::env::{Env, env_new, env_get, env_set, env_set_alias, env_root};

enum FormType {
    Def,
    Let,
    Do,
    If,
    Fn,
    Eval,
    DefAlias,
    Function,
}

impl FormType {
    pub fn from(name: &str) -> FormType {
        match name {
            "def!" => FormType::Def,
            "let" => FormType::Let,
            "do" => FormType::Do,
            "if" => FormType::If,
            "fn" => FormType::Fn,
            "eval" => FormType::Eval,
            "defalias!" => FormType::DefAlias,
            _ => FormType::Function,
        }
    }
}

pub fn eval(ast: LispValue, env: Env) -> LispResult {
    match *ast {
        LispType::List(_) => return eval_list(ast, env),
        _ => return eval_ast(ast, env),
    };
}

fn eval_list(ast: LispValue, env: Env) -> LispResult {
    let (form_type, elements) = match *ast {
        LispType::List(ref elements) => {
            let ref head = *elements[0];
            match *head {
                LispType::Symbol(ref name) => (FormType::from(name), elements),
                _ => (FormType::Function, elements),
            }
        },
        _ => panic!("unreachable code"),
    };

    match form_type {
        FormType::Def => eval_def(elements, env),
        FormType::Let => eval_let(elements, env),
        FormType::Do => eval_do(elements, env),
        FormType::If => eval_if(elements, env),
        FormType::Fn => eval_fn(elements, env),
        FormType::Eval => eval_eval(elements, env),
        FormType::DefAlias => eval_alias(elements, env),
        FormType::Function => eval_function(ast.clone(), env),
    }
}

fn eval_ast(ast: LispValue, env: Env) -> LispResult {
    match *ast {
        LispType::Symbol(_) => env_get(&env, &ast),
        LispType::List(ref a) | LispType::Vector(ref a) => {
            let mut ast_vec : Vec<LispValue> = vec![];
            for mv in a.iter() {
                let mv2 = mv.clone();
                ast_vec.push(try!(eval(mv2, env.clone())));
            }
            Ok(match *ast { LispType::List(_) => list(ast_vec),
            _                          => vector(ast_vec) })
        },
        _ => Ok(ast.clone()),
    }
}

fn eval_def(elements: &Vec<LispValue>, env: Env) -> LispResult {
    let a1 = (*elements)[1].clone();
    let a2 = (*elements)[2].clone();
    let r = try!(eval(a2, env.clone()));
    match *a1 {
        LispType::Symbol(_) => {
            env_set(&env.clone(), a1, r.clone());
            return Ok(_nil());
        },
        _ => panic!("def! of non-symbol"),
    }
}

fn eval_let(elements: &Vec<LispValue>, env: Env) -> LispResult {
    let let_env = env_new(Some(env.clone()));
    let a1 = (*elements)[1].clone();
    let a2 = (*elements)[2].clone();
    match *a1 {
        LispType::List(ref binds) | LispType::Vector(ref binds) => {
            let mut it = binds.iter();
            while it.len() >= 2 {
                let b = it.next().unwrap();
                let exp = it.next().unwrap();
                match **b {
                    LispType::Symbol(_) => {
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

fn eval_do(elements: &Vec<LispValue>, env: Env) -> LispResult {
    if elements.len() < 2 {
        return Ok(_nil());
    }

    let el = list(elements[1..].to_vec());
    match *try!(eval_ast(el, env.clone())) {
        LispType::List(ref lst) => {
            let ref last = lst[lst.len()-1];
            return Ok(last.clone());
        }
        _ => panic!("invalid do call"),
    }
}


fn eval_if(elements: &Vec<LispValue>, env: Env) -> LispResult {
    let a1 = (*elements)[1].clone();
    let c = try!(eval(a1, env.clone()));
    match *c {
        LispType::False | LispType::Nil => {
            if elements.len() >= 4 {
                let a3 = (*elements)[3].clone();
                return eval(a3, env.clone());
            } else {
                return Ok(_nil());
            }
        },
        _ => {
            let a2 = (*elements)[2].clone();
            return eval(a2, env.clone());
        },
    }
}

fn eval_fn(elements: &Vec<LispValue>, env: Env) -> LispResult {
    let a1 = elements[1].clone();
    let a2 = elements[2].clone();
    return Ok(function(eval, a2, env, a1));
}

fn eval_eval(elements: &Vec<LispValue>, env: Env) -> LispResult {
    let a1 = (*elements)[1].clone();
    let ast = try!(eval(a1, env.clone()));
    let env = env_root(&env);
    return eval(ast, env);
}


fn eval_alias(elements: &Vec<LispValue>, env: Env) -> LispResult {
    let name = (*elements)[1].clone();
    let target = (*elements)[2].clone();
    match *name {
        LispType::Symbol(ref name_value) => {
            match *target {
                LispType::Symbol(ref target_value) => {
                    env_set_alias(&env.clone(), name_value, target_value);
                    return Ok(_nil());
                },
                _ => panic!("defalias!: syntax error"),
            }
        },
        _ => panic!("defalias!: syntax error"),
    }
}

fn eval_function(ast: LispValue, env: Env) -> LispResult {
    let el = try!(eval_ast(ast.clone(), env.clone()));
    let args = match *el {
        LispType::List(ref args) => args,
        _ => panic!("unreachable code"),
    };
    let ref f = args.clone()[0];
    f.apply(args[1..].to_vec())
}
