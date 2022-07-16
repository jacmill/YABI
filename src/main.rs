struct Token {
    value: char,
    type_: TokenType,
    pos: u32 
}

enum TokenType {
    MovePointerRight,
    MovePointerLeft,
    IncrementCell,
    DecrementCell,
    OutputCell,
    InputCell,
    BeginLoop,
    EndLoop,
    EOF
}

struct Lexer {
    input: String
}
impl Lexer {
    fn lex(&self) -> Vec<Token> {
        todo!()
    }
}
fn main() {
    println!("Hello, world!");
}
