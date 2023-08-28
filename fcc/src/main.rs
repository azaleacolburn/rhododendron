use clap::Parser;
use std::os::raw::c_int;    // 32 bits
use std::os::raw::c_char;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

// Reads a file, next try integration with the C compiler
// I suppose this function will actually call the compiler
fn main() -> &str {
    let args = Cli::parse();
    let content = match std::fs::read_to_string(&args.path) {
        Ok(content) => content,
        Err(err) => panic!(err)
    };

    &contents
}
