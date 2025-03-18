use std::collections::{HashMap, HashSet};
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
    Operator(Operators),
}

enum Operators {
    Add,
    Subtract,
    Multiply,
    Divide
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
    let mut stmt = conn.prepare("SELECT token FROM words WHERE keyword =?1")?;
    let token = stmt.query_row([word], |row| row.get(0))?;
    tokenize_keyword(token).ok_or_else(|| rusqlite::Error::InvalidQuery)
}

pub struct Token {
    value: &'static str,
    token: Tokens,
}

impl Token {
    pub fn new(value: &str, token: Tokens) -> Token {
        Token { value, token }
    }
}

pub fn tokenizer(code: &str) -> HashMap<i32, Token> {
    let sep_code: Vec<&str> = split_string(code);
    let mut result: HashMap<i32, Token> = HashMap::new();

    // Add more keywords, also to the database
    let keywords: HashSet<&str> = ["let", "int", "float", "long", "double"].iter().cloned().collect();

    for word in sep_code {
        // check if its only letters without signs
        if word.chars().all(|c| {c.is_ascii_alphanumeric()}){
            let token = if keywords.contains(word){
                is_keyword(word).unwrap_or(Tokens::Identifier)
            } else if let Ok(_) = word.parse::<i32>(){
                Tokens::Number
            }else{
                Tokens::Identifier
            };
            result.insert(result.len() as i32, Token::new(word, token));
        } else {
            for char in word.chars(){
                let token = match char {
                    '(' => Tokens::OpenParen,
                    ')' => Tokens::CloseParen,
                    '{' => Tokens::OpenBrace,
                    '}' => Tokens::CloseBrace,
                    '+' => Tokens::Operator(Operators::Add),
                    '-' => Tokens::Operator(Operators::Subtract),
                    '*' => Tokens::Operator(Operators::Multiply),
                    '/' => Tokens::Operator(Operators::Divide),
                    _ => Tokens::Identifier
                };
                result.insert(result.len() as i32, Token::new(&char.to_string(), token));
            }
        }
    }
    result
}

fn split_string(code: &str) -> Vec<&str> {
    let sep_code: Vec<&str> = code.trim().split_whitespace().collect();
    sep_code
}
