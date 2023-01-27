extern crate core;

use std::cell::RefCell;
use std::rc::Rc;
use crate::evaluator::eval;
use crate::meta::Meta;
use crate::object::Object;

mod lexzer;
mod parser;
mod object;
mod evaluator;
mod meta;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut meta = Rc::new(RefCell::new(Meta::new()));

    loop {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input).unwrap();
        if input.eq("exit") {
            println!("GOOD BYE!!!");
            break
        }
        let val = eval(input.as_ref(), &mut meta)?;
        match val {
            Object::Void => {},
            Object::Integer(n) => println!("res: {}", n),
            Object::Bool(b) => println!("res: {}", b),
            Object::Symbol(s) => println!("res: {}", s),
            Object::Lambda(params, body) => {
                println!("Lamdbda");
                for param in params {
                    println!("{}", param);
                }
                println!(")");
                for expr in body {
                    println!("{}", expr);
                }
            }
            _ => println!("{}", val),
        }
    }
    Ok(())
}
