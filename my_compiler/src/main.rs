mod lexer;

fn main() {
    lexer::tokenizer("int main(){let hello = 3}");
}
