pub fn entry() {
    println!("Hello, world!");
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Token {
    IDENT(Vec<char>),
    NUM(Vec<char>),
    OP(Vec<char>),
    DELIMITER(Vec<char>),
    STRING(Vec<char>),
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum AST {
    Seq(Vec<Box<AST>>),
    String(Token),
    Empty
}

impl AST {
    pub fn new() -> AST {
        AST::Empty
    }
}

pub fn render(ss: String) -> String {
    unimplemented!();
}
