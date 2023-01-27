use std::error::Error;
use std::fmt::{Display, Formatter, write};

#[derive(Debug, Clone, PartialEq)]
pub enum Analyzer {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}

impl Display for Analyzer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Analyzer::Integer(n) => write!(f, "{}", n),
            Analyzer::Symbol(s) => write!(f, "{}", s),
            Analyzer::LParen => write!(f, "("),
            Analyzer::RParen => write!(f, ")"),
        }
    }
}

#[derive(Debug)]
pub struct AnalyzerError {
    ch: char
}

impl Error for AnalyzerError {}

impl Display for AnalyzerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unexpected character: {}", self.ch)
    }
}

pub fn analyzer(param: &str) -> Result<Vec<Analyzer>, AnalyzerError> {
    let check_param = param.replace("(", " ( ").replace(")", " ) ");
    let words = check_param.split_whitespace();
    let mut tokens: Vec<Analyzer> = Vec::new();
    for word in words {
        match word {
            "(" => tokens.push(Analyzer::LParen),
            ")" => tokens.push(Analyzer::RParen),
            _ => {
                let item = word.parse::<i64>();
                if item.is_ok() {
                    tokens.push(Analyzer::Integer(item.unwrap()));
                } else {
                    tokens.push(Analyzer::Symbol(word.to_string()));
                }
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_add() {
        let param = analyzer("(+ 1 2)").unwrap_or(vec![]);
        assert_eq!(param,
                   vec![
                       Analyzer::LParen,
                       Analyzer::Symbol("+".to_string()),
                       Analyzer::Integer(1),
                       Analyzer::Integer(2),
                       Analyzer::RParen,
                   ]);
    }
}