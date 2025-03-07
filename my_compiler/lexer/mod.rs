pub enum Tokens {
    number,
    identifier,
    equals,
    openParen,
    closeParen,
    binaryOperator,
    variable,
}

pub struct Token {
    value: String,
    token: Tokens,
}

pub fn tokenizer(code: &str) -> HashMap<i32, Token> {
    let sepCode: Vec<&str> = splitString(code);

    while !sepCode.is_empty() {}
}

fn splitString(code: &str) -> Vec<&str> {
    let sepCode: Vec<&str> = code.trim().split(' ').collect();
    sepCode
}
