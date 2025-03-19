use std::collections::{HashMap, HashSet};
use rusqlite::{Connection};
#[derive(Debug)]
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
#[derive(Debug)]
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

fn is_keyword(word: String) -> Result<Tokens, rusqlite::Error> {
    let conn = Connection::open("keywords.db")?;
    let mut stmt = conn.prepare("SELECT token FROM words WHERE keyword =?1")?;
    let token: String = stmt.query_row([word], |row| row.get(0))?;
    tokenize_keyword(&token).ok_or_else(|| rusqlite::Error::InvalidQuery)
}
#[derive(Debug)]
pub struct Token {
    value: String,
    token: Tokens,
}

impl Token {
    pub fn new(value: String, token: Tokens) -> Token {
        Token { value, token }
    }
}


pub fn tokenizer(code: String) -> HashMap<i32, Token> {
    let sep_code: Vec<String> = split_string(code);
    let mut result: HashMap<i32, Token> = HashMap::new();

    // Add more keywords, also to the database
    let keywords: HashSet<&str> = ["let", "int", "float", "long", "double"].iter().cloned().collect();

    for word in sep_code {
        // check if its only letters without signs
        if word.chars().all(|c| {c.is_ascii_alphanumeric()}){
            let token = if keywords.contains(&*word){
                // Add that the word after this one is variables name
                is_keyword(word.clone()).unwrap_or(Tokens::Identifier)
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
                    '=' => Tokens::Equals,
                    '+' => Tokens::Operator(Operators::Add),
                    '-' => Tokens::Operator(Operators::Subtract),
                    '*' => Tokens::Operator(Operators::Multiply),
                    '/' => Tokens::Operator(Operators::Divide),
                    _ => Tokens::Identifier
                };
                result.insert(result.len() as i32, Token::new(char.to_string(), token));
            }
        }
    }
    result
}

fn split_string(code: String) -> Vec<String> {
    let sep_code: Vec<String> = code.trim().split_whitespace().map(|s| s.to_string()).collect();
    sep_code
}
