mod lexer;

fn main() {
    println!("{:?}",lexer::tokenizer("int main (){ int hello = 3 }".to_string()));
}
