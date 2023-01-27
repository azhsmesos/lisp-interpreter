extern crate core;

use std::cell::RefCell;
use std::rc::Rc;
use linefeed::{Interface, ReadResult};
use crate::evaluator::eval;
use crate::meta::Meta;
use crate::object::Object;

mod lexzer;
mod parser;
mod object;
mod evaluator;
mod meta;

const APPLICATION: &str = "lisp-sh";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = Interface::new(APPLICATION)?;
    let mut meta = Rc::new(RefCell::new(Meta::new()));
    reader.set_prompt(APPLICATION)?;

    while let ReadResult::Input(input) = reader.read_line()? {
        if input.eq("exit") {
            break
        }
        let val = eval(input.as_ref(), &mut meta)?;
        match val {
            Object::Void => {},
            Object::Integer(n) => println!("{}", n),
            Object::Bool(b) => println!("{}", b),
            Object::Symbol(s) => println!("{}", s),
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
    println!("GOOD BYE!!!");
    Ok(())
}
