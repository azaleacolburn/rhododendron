#[allow(unused_assignments)]
mod lexer;
mod parser;
mod code_gen;
use lexer::{Token, VariableTypes};

pub fn main() {
    let buff = String::from("int my_int = 1 + 1;");
    // let tokens = lexer::string_to_tokens(buff).unwrap();
    // let lexed_tokens: Vec<Token> = lexer::string_to_tokens(&buff).unwrap();

    // for tok in &lexed_tokens {
        // println!("{:?}", tok);
    // }
    // int my_int = 5 * 2 + 3;
    let tokens = vec![Token::Type(VariableTypes::Int), Token::Id(String::from("my_int")), Token::Eq, Token::NumLiteral(5), Token::Star, Token::NumLiteral(2), Token::Add, Token::NumLiteral(3), Token::Add, Token::NumLiteral(4), Token::Semi];
    let node = parser::program(&tokens).unwrap();
    node.print(&mut 0);
    code_gen::code_gen(&node);
    assert_eq!(lexer::string_to_tokens(&buff), lexer::string_to_tokens(&String::from("int my_int=1+1;")));

}