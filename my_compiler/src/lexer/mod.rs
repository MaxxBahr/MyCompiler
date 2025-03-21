use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::exit;

static HADERROR: bool = false;

#[derive(Debug)]
pub enum Tokens {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
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
    if HADERROR {
        exit(0);
    }
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
        &HADERROR = &false;


    }
}

fn run(input: String){
    let mut scanner = Scanner::new(input);
    let tokens: Vec<Token> = scanner.scanTokens();

    for token in tokens{
        println!("{:?}", token);
    }
}

pub fn error(line: i32, message: String){
    report(line, "".to_string(), message);
}

pub fn report(line: i32, here: String, message: String){
    println!("[line: {line}] Error {here}: {message}");
    &HADERROR = &true;
}

