use std::collections::HashMap;
use rusqlite::{Connection};
pub enum Tokens {
    Number,
    Identifier,
    Equals,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    BinaryOperator,
    VariableName,
    Variable,
    ReturnValue,
}

fn tokenize_keyword(token: &str) -> Option<Tokens>{
    match token {
        "Variable" => Some(Tokens::Variable),
        "ReturnValue" => Some(Tokens::ReturnValue),
        _ => None,
    }
}

fn is_keyword(word: &str) -> Result<Tokens, rusqlite::Error> {
    let conn = Connection::open("keywords.db")?;
    let mut stmt = conn.prepare("SELECT token FROM words WHERE word =?1")?;
    let token = stmt.query_row([word], |row| row.get(0))?;
    tokenize_keyword(token).ok_or_else(|| rusqlite::Error::InvalidQuery)
}

pub struct Token {
    value: String,
    token: Tokens,
}

impl Token {
    pub fn new(value: String, token: Tokens) -> Token {
        Token { value, token }
    }
}

pub fn tokenizer(code: &str) -> HashMap<i32, Token> {
    let sep_code: Vec<&str> = split_string(code);
    let mut result: HashMap<i32, Token> = HashMap::new();

    for word in sep_code {
        if let Ok(number) = word.parse::<i32>() {
            result.insert(result.len(), Token::new(word, Tokens::Number));
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
        // If there is a keyword like let we have to store the VariableNames name

        // Map VariableName, operator, braces, parantheses, Identifier
    }
    result
}

fn split_string(code: &str) -> Vec<&str> {
    let sep_code: Vec<&str> = code.trim().split_whitespace().collect();
    sep_code
}
