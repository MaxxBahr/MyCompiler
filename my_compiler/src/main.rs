mod lexer;

fn main() {
    println!("{:?}",lexer::tokenizer("int main (){ let hello = 3 }".to_string()));
}
