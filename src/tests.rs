use crate::{lexer::{string_to_tokens, self}, parser::program, code_gen::code_gen};
use std::{fs, io, path::PathBuf, str, string};

#[test]
fn test_all() {
    let tests = match get_all_files("../tests/rh") {
        Ok(tests) => tests,
        Err(_) => panic!("No files to be tests")
    };
    let validation = match get_all_files("../tests/asm") {
        Ok(validation) => validation,
        Err(_) => panic!("No validation files")
    };
    assert_eq!(tests.len(), validation.len());
    for test_i in 0..tests.len() {
        let raw_src = fs::read(tests[test_i].clone()).unwrap();
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
        let valid_raw = fs::read(validation[test_i].clone()).expect("a corresponding validation test");
        let valid = str::from_utf8(&valid_raw).unwrap().to_string();
        // Check the generated asm against the expected asm
        assert_eq!(asm, valid);
    }
}

fn get_all_files(path: &str) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;
    let all: Vec<PathBuf> = entries
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();
    Ok(all)
}