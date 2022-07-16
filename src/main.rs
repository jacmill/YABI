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
    EndLoop
}
fn main() {
    println!("Hello, world!");
}
