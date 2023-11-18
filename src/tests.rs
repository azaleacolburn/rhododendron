use crate::{lexer::{string_to_tokens, self}, parser::program, code_gen::code_gen};
use std::{fs, io, path::PathBuf, str, string};

#[test]
fn test_all() {
    let tests = match get_all_test("../tests/") {
        Ok(tests) => tests,
        Err(_) => panic!("No files to be tests")
    };
    for test in tests {
        let raw_src = fs::read(test).unwrap();
        let src = str::from_utf8(&raw_src).unwrap();
        let tokens = match string_to_tokens(src) {
            Ok(tokens) => tokens,
            Err(_) => panic!("Unable to parse integer literal")
        };
        let node = match program(&tokens) {
            Ok(node) => node,
            Err(err) => panic!("Failed Parsing: {:?}", err)
        };
        let asm = code_gen(&node);
        // Check the generated asm against the expected asm
        // assert_eq!() 
    }
}

fn get_all_test(path: &str) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;
    let all: Vec<PathBuf> = entries
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();
    Ok(all)
}