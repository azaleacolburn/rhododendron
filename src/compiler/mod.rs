mod ast;
mod code_gen;
mod error;
mod lexer;
mod linker;
mod parser;

use clap::{ArgAction, Parser};
use std::sync::Mutex;

static DEBUG: Mutex<bool> = Mutex::new(true);

#[macro_export]
macro_rules! dbg_println {
    ($($arg:tt)*) => {
        if *DEBUG.lock().unwrap() {
            println!($($arg)*);
        }
    };
}

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
    let mut lock = DEBUG.lock().expect("Failed to get lock on DEBUG Mutex");
    *lock = args.debug;
    drop(lock);
    test(file_name, exe_name);
}

pub fn test(file_name: std::path::PathBuf, exe_name: String) {
    let buff = std::fs::read_to_string(file_name).expect("Source file must exist");
    println!("test1");
    dbg_println!("test");
    println!("test2");
    let (tokens, line_tracker) = lexer::string_to_tokens(&buff).unwrap();
    println!("{:?}", tokens);
    let node = parser::program(tokens, line_tracker).unwrap();
    node.print(&mut 0);
    let code = code_gen::main(&node);
    dbg_println!("{}", code);
    let _ = std::fs::write(exe_name, code);
}
