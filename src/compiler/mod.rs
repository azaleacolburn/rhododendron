mod code_gen;
mod error;
#[allow(unused_assignments)]
mod lexer;
mod linker;
mod parser;
use std::sync::Mutex;

use clap::{ArgAction, Parser};

static DEBUG: Mutex<bool> = Mutex::new(false);

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
    test(file_name, exe_name);
}

pub fn test(file_name: std::path::PathBuf, exe_name: String) {
    let buff = std::fs::read_to_string(file_name).expect("Source file must exist");
    let (tokens, line_tracker) = lexer::string_to_tokens(&buff).unwrap();
    let node = parser::program(tokens, line_tracker).unwrap();
    let code = code_gen::main(&node);
    dbg_println!("{}", code);
    let _ = std::fs::write(exe_name, code);
}
