mod code_gen;
mod error;
#[allow(unused_assignments)]
mod lexer;
mod linker;
mod parser;
use clap::{ArgAction, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: std::path::PathBuf,
    name: String,
    #[clap(long, short, action=ArgAction::SetFalse)]
    debug: bool,
}

pub fn main() {
    let args = Args::parse();
    let file_name: std::path::PathBuf = args.path;
    let exe_name: String = args.name;
    test(file_name, exe_name, args.debug);
}

pub fn test(file_name: std::path::PathBuf, exe_name: String, debug: bool) {
    let buff = std::fs::read_to_string(file_name).expect("Source file must exist");
    let (tokens, line_tracker) = lexer::string_to_tokens(&buff).unwrap();
    let node = parser::program(tokens, line_tracker, debug).unwrap();
    let code = code_gen::main(&node, debug);
    println!("{}", code);
    let _ = std::fs::write(exe_name, code);
}
