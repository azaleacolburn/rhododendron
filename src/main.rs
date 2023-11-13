#[allow(unused_assignments)]
mod lexer;
mod parser;
mod code_gen;
use lexer::{Token, VariableTypes};

pub fn main() {
    let buff = String::from("int i = 1 + 1;");
    // let tokens = lexer::string_to_tokens(buff).unwrap();
    let tokens = vec![Token::Type(VariableTypes::Int), Token::Id(String::from("my_int")), Token::Eq, Token::NumLiteral(1), Token::Add, Token::NumLiteral(1), Token::Semi];
    let node = parser::program(&tokens).unwrap();
    node.print();
    code_gen::code_gen(node);
}