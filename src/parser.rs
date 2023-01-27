use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::lexzer::{analyzer, Analyzer};
use crate::object::Object;

#[derive(Debug)]
pub struct ParseError {
    err: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {}", self.err)
    }
}

impl Error for ParseError {}

pub fn parse(param: &str) -> Result<Object, ParseError> {
    let analyzer_result = analyzer(param);
    if analyzer_result.is_err() {
        return Err(ParseError {
            err: format!("{}", analyzer_result.err().unwrap())
        });
    }

    let mut analyzer_str = analyzer_result
        .unwrap()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();

    let parse_list = parse_list(&mut analyzer_str)?;
    Ok(parse_list)
}

fn parse_list(analyzer: &mut Vec<Analyzer>) -> Result<Object, ParseError> {
    let token = analyzer.pop();
    if token != Some(Analyzer::LParen) && token != Some(Analyzer::EXIT) {
        return Err(ParseError {
            err: format!("Expected LParen( ( ), found {:?}", token)
        });
    }

    let mut list: Vec<Object> = Vec::new();
    while !analyzer.is_empty() {
        let token = analyzer.pop();
        if token == None {
            return Err(ParseError {
                err: format!("did not find enough token")
            });
        }
        let now_token = token.unwrap();
        match now_token {
            Analyzer::Integer(n) => list.push(Object::Integer(n)),
            Analyzer::Symbol(s) => list.push(Object::Symbol(s)),
            Analyzer::LParen => {
                analyzer.push(Analyzer::LParen);
                let sub_list = parse_list(analyzer)?;
                list.push(sub_list);
            }
            Analyzer::RParen => {
                return Ok(Object::List(list))
            }
            _ => return Ok(Object::List(list))
        }
    }

    Ok(Object::List(list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let param = parse("(+ 1 2)").unwrap();
        println!("{:?}", param);
    }

    #[test]
    fn test_parse_analyzer() {
        let param = parse("(+ 1 2)").unwrap();
        assert_eq!(param,
                   Object::List(vec![
                       Object::Symbol("+".to_string()),
                       Object::Integer(1),
                       Object::Integer(2),
                   ]));
    }

    #[test]
    fn test_parse_analyzer2() {
        let param = parse("(+ 1 2 3 4)").unwrap();
        assert_eq!(param,
                   Object::List(vec![
                       Object::Symbol("+".to_string()),
                       Object::Integer(1),
                       Object::Integer(2),
                       Object::Integer(3),
                       Object::Integer(4),
                   ]));
    }
}