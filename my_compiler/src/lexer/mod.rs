use std::collections::{HashMap, HashSet};
use std::path::Path;
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
    VariableType,
    FunctionName,
    Operator(Operators),
}
#[derive(Debug)]
enum Operators {
    Add,
    Subtract,
    Multiply,
    Divide
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

pub fn lexer(args: String){
    if (args.len() > 1){
        println!("Usage: MyCompiler [script]");
    }else if args.len() == 1{
        runFile(args);
    }else{
        runPrompt();
    }
}

fn runFile(args: String){
    let code = std::io::read_to_string(Path::new(&args)).unwrap();
    run(code);
}

fn runPrompt(){
    use std::io::{stdin, stdout, Write};
    let mut input = String::new();
    loop {
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Did not enter correct String");
        if let Some('\n') = input.chars().next_back(){
            input.pop();
        }
        if let Some('\r') = input.chars().next_back(){
            input.pop();
        }
        if input.is_empty(){break;}
        run(input.clone());
    }
}

fn run(input: String){
    let mut scanner = Scanner::new(input);
    let tokens:
}