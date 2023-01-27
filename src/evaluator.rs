use std::cell::RefCell;
use std::rc::Rc;
use crate::meta::Meta;
use crate::object::Object;
use crate::parser::parse;

pub fn eval(param: &str, meta: &mut Rc<RefCell<Meta>>) -> Result<Object, String> {
    let param_list = parse(param);
    if param_list.is_err() {
        return Err(format!("{}", param_list.err().unwrap()));
    }

    eval_obj(&param_list.unwrap(), meta)
}

fn eval_obj(obj: &Object, meta: &mut Rc<RefCell<Meta>>) -> Result<Object, String> {
    match obj {
        Object::Void => Ok(Object::Void),
        Object::Integer(n) => Ok(Object::Integer(*n)),
        Object::Bool(_) => Ok(obj.clone()),
        Object::Symbol(s) => eval_symbol(s, meta),
        Object::Lambda(_params, _body) => Ok(Object::Void),
        Object::List(list) => eval_list(list, meta),
    }
}

fn eval_symbol(s: &str, meta: &mut Rc<RefCell<Meta>>) -> Result<Object, String> {
    let val = meta.borrow_mut().get(s);
    if val.is_none() {
        return Err(format!("unbound symbol: {}", s));
    }
    Ok(val.unwrap().clone())
}

fn eval_list(list: &Vec<Object>, meta: &mut Rc<RefCell<Meta>>) -> Result<Object, String> {
    let head = &list[0];
    match head {
        Object::Symbol(s) => match s.as_str() {
            "+" | "-" | "/" | "<" | ">" | "=" | "!=" => {
                return eval_binary_op(&list, meta);
            },
            "define" => eval_define(&list, meta),
            "if" => eval_if(&list, meta),
            "lambda" => eval_function_definition(&list),
            _ => eval_function_call(&s, &list, meta),
        },
        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let res = eval_obj(obj, meta)?;
                match res {
                    Object::Void => {},
                    _ => new_list.push(res),
                }
            }
            Ok(Object::List(new_list))
        }
    }
}

fn eval_function_call(s: &str, list: &Vec<Object>, meta: &mut Rc<RefCell<Meta>>) -> Result<Object, String> {
    let lambda = meta.borrow_mut().get(s);
    if lambda.is_none() {
        return Err(format!("unbound symbol: {}", s));
    }
    let func = lambda.unwrap();
    match func {
        Object::Lambda(params, body) => {
            let mut new_meta = Rc::new(RefCell::new(Meta::extend(meta.clone())));
            for (i, param) in params.iter().enumerate() {
                let val = eval_obj(&list[i + 1], meta)?;
                new_meta.borrow_mut().set(param, val);
            }
            return eval_obj(&Object::List(body), &mut new_meta);
        },
        _ => return Err(format!("not a lambda: {}", s)),
    }
}

fn eval_binary_op(list: &Vec<Object>,  meta: &mut Rc<RefCell<Meta>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("invalid number of arguments for infix operator"));
    }
    let operator = list[0].clone();
    let left = eval_obj(&list[1].clone(), meta)?;
    let right = eval_obj(&list[2].clone(), meta)?;
    let left_val = match left {
        Object::Integer(n) => n,
        _ => return Err(format!("left operation must be an integer {:?}", left)),
    };
    let right_val = match right {
        Object::Integer(n) => n,
        _ => return Err(format!("right operation must be an integer {:?}", right)),
    };
    match operator {
        Object::Symbol(s) => match s.as_str() {
            "+" => Ok(Object::Integer(left_val + right_val)),
            "-" => Ok(Object::Integer(left_val - right_val)),
            "*" => Ok(Object::Integer(left_val * right_val)),
            "/" => Ok(Object::Integer(left_val / right_val)),
            "<" => Ok(Object::Bool(left_val < right_val)),
            ">" => Ok(Object::Bool(left_val > right_val)),
            "=" => Ok(Object::Bool(left_val == right_val)),
            "!=" => Ok(Object::Bool(left_val != right_val)),
            _ => Err(format!("invalid infix operator: {}", s)),
        },
        _ => Err(format!("operator must be a symbol")),
    }
}

fn eval_define(list: &Vec<Object>, meta: &mut Rc<RefCell<Meta>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("invalid number of arguments for infix operator"));
    }
    let sym = match &list[1] {
        Object::Symbol(s) => s.clone(),
        _ => return Err(format!("invalid define")),
    };
    let val = eval_obj(&list[2], meta)?;
    meta.borrow_mut().set(&sym, val);
    Ok(Object::Void)
}

fn eval_if(list: &Vec<Object>, meta: &mut Rc<RefCell<Meta>>) -> Result<Object, String> {
    if list.len() != 4 {
        return Err(format!("invalid number of arguments for infix operator"));
    }
    let obj = eval_obj(&list[1], meta)?;
    let real_obj = match obj {
        Object::Bool(b) => b,
        _ => return Err(format!("condition must be a boolean")),
    };
    return if real_obj == true {
        eval_obj(&list[2], meta)
    } else {
        eval_obj(&list[3], meta)
    }
}

fn eval_function_definition(list: &Vec<Object>) -> Result<Object, String> {
    let params = match &list[1] {
        Object::List(list) => {
            let mut params = Vec::new();
            for param in list {
                match param {
                    Object::Symbol(s) => params.push(s.clone()),
                    _ => return Err(format!("invalid lambda parameter")),
                }
            }
            params
        }
        _ => return Err(format!("invalid lambda")),
    };
    let body = match &list[2] {
        Object::List(list) => list.clone(),
        _ => return Err(format!("invalid lambda")),
    };
    Ok(Object::Lambda(params, body))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_eval_binary_op() {
        let mut meta = Rc::new(RefCell::new(Meta::new()));
        let item = eval("(+ 1 2)", &mut meta);
        println!("item: {:?}", item);
    }

    #[test]
    fn test_area_of_a_circle() {
        let mut env = Rc::new(RefCell::new(Meta::new()));
        let program = "(
                        (define r 10)
                        (define pi 314)
                        (* pi (* r r))
                      )";
        let result = eval(program, &mut env).unwrap();
        println!("res: {:?}", result);
        // assert_eq!(
        //     result,
        //     Object::List(vec![Object::Integer((314 * 10 * 10) as i64)])
        // );
    }
}