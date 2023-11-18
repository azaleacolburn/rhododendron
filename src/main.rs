#[allow(unused_assignments)]
mod lexer;
mod parser;
mod code_gen;
#[cfg(test)]
mod tests;
use lexer::{Token,   VariableTypes};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    path: std::path::PathBuf,
    name: String
}

pub fn main() {
    let args = Args::parse();
    let file_name: std::path::PathBuf = args.path;
    let exe_name: String = args.name;
    let buff = match std::fs::read_to_string(file_name) {
        Ok(contents) => contents,
        Err(err) => panic!("File does not exist")
    }; 
    // let buff = String::from("int my_int = 1 + 1;");
    let tokens = lexer::string_to_tokens(&buff).unwrap();
    println!("Past lexing");
    // let lexed_tokens: Vec<Token> = lexer::string_to_tokens(&buff).unwrap();

    // for tok in &lexed_tokens {
        // println!("{:?}", tok);
    // }
    // int my_int = 5 * 2 + 3;
    // let tokens = vec![Token::Type(VariableTypes::Int), Token::Id(String::from("my_int")), Token::Eq, Token::NumLiteral(5), Token::Star, Token::NumLiteral(2), Token::Add, Token::NumLiteral(3), Token::Add, Token::NumLiteral(4), Token::Semi];
    let node = parser::program(&tokens).unwrap();
    node.print(&mut 0);
    println!("past ast");
    let code = code_gen::code_gen(&node);
    let _ = std::fs::write(exe_name, code);
    // assert_eq!(lexer::string_to_tokens(&buff), lexer::string_to_tokens(&String::from("int my_int=1+1;")));

}