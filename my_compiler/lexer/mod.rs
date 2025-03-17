use std::collections::HashMap;
pub enum Tokens {
    Number,
    Identifier,
    Equals,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    BinaryOperator,
    Variable,
}

pub struct Token {
    value: String,
    token: Tokens,
}

impl Token {
    pub fn new(value: String, token: String) -> Token {
        Token { value, token }
    }
}

pub fn tokenizer(code: &str) -> HashMap<i32, Token> {
    let sep_code: Vec<&str> = split_string(code);
    let mut result: HashMap<i32, Token>;

    for word in sep_code {
        if let Ok(number) = word.parse::<i32>() {
            result.insert(result.len(), Token::new(number, Tokens::Number));
        }
        if word == "("{
            result.insert(result.len(), Token::new(word, Tokens::OpenParen));
        }
        if word == ")"{
            result.insert(result.len(), Token::new(word, Tokens::CloseParen));
        }
        if word == "{"{
            result.insert(result.len(), Token::new(word, Tokens::OpenBrace));
        }
        if word == "}"{
            result.insert(result.len(), Token::new(word, Tokens::CloseBrace));
        }
        // If there is a keyword like let we have to store the variables name
        // Map Variable, operator, braces, parantheses, Identifier
    }
}

fn split_string(code: &str) -> Vec<&str> {
    let sep_code: Vec<&str> = code.trim().split(' ').collect();
    sep_code
}
