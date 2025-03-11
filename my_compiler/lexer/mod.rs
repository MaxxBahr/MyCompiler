pub enum Tokens {
    number,
    identifier,
    equals,
    openParen,
    closeParen,
    openBrace,
    closeBrace,
    binaryOperator,
    variable,
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
    let sepCode: Vec<&str> = splitString(code);
    let mut result: HashMap<i32, Token>;

    for word in sepCode {
        if let Ok(number) = word.parse::<i32>() {
            result.insert(result.len(), Token::new(number, Tokens::number));
        }
        // Map variable, operator, braces, parantheses, identifier
    }
}

fn splitString(code: &str) -> Vec<&str> {
    let sepCode: Vec<&str> = code.trim().split(' ').collect();
    sepCode
}
