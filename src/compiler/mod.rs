mod code_gen;
mod error;
#[allow(unused_assignments)]
mod lexer;
mod linker;
mod parser;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: std::path::PathBuf,
    name: String,
}

pub fn main() {
    let args = Args::parse();
    let file_name: std::path::PathBuf = args.path;
    let exe_name: String = args.name;
    test(file_name, exe_name);
}

pub fn test(file_name: std::path::PathBuf, exe_name: String) {
    let buff = match std::fs::read_to_string(file_name) {
        Ok(contents) => contents,
        Err(_) => panic!("File does not exist"),
    };
    let (tokens, line_tracker) = lexer::string_to_tokens(&buff).unwrap();
    let node = parser::program(tokens, line_tracker).unwrap();
    let code = code_gen::main(&node);
    println!("{}", code);
    let _ = std::fs::write(exe_name, code);
}

