use std::fmt;
use std::vec;

struct Token {
    value: String,
    type_: TokenType,
    pos: usize 
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "type: {:?} with value: {} at pos: {}", self.type_, self.value, self.pos)
    }
}
#[derive(Debug)]
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
        let mut tokens: Vec<Token> = vec![];
        for (pos, lookahead) in self.input.chars().enumerate() {
            let token_type: TokenType = match lookahead {
                '>' => TokenType::MovePointerRight,
                '<' => TokenType::MovePointerLeft,
                '+' => TokenType::IncrementCell,
                '-' => TokenType::DecrementCell,
                '.' => TokenType::OutputCell,
                ',' => TokenType::InputCell,
                '[' => TokenType::BeginLoop,
                ']' => TokenType::EndLoop,
                _ => continue
            };
            tokens.push(Token{
                value: lookahead.to_string(),
                type_: token_type,
                pos: pos
            });
        }
        tokens.push(Token{
            value: String::from("<EOF>"),
            type_: TokenType::EOF,
            pos: self.input.len()
        });

        tokens
    }
}
fn main() {
    let tokens = Lexer{input: String::from("++++>>>+>---")}.lex();
    for token in tokens {
        println!("{}", token);
    } 
}
