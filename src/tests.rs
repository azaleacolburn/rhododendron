use crate::{code_gen::main, lexer::string_to_tokens, parser::program};
use std::{
    fs, io,
    path::{Path, PathBuf},
    str,
};

#[test]
fn test_all() {
    let tests: Vec<PathBuf> = match get_all_files(&Path::new("tests/rh/")) {
        Ok(tests) => tests,
        Err(_) => panic!("No files to be tests"),
    };
    let validation: Vec<PathBuf> = match get_all_files(&Path::new("tests/asm/")) {
        Ok(validation) => validation,
        Err(_) => panic!("No validation files"),
    };
    assert_eq!(tests.len(), validation.len());

    for test_i in 0..tests.len() {
        let raw_src = fs::read(tests[test_i].clone()).unwrap();
        let src = str::from_utf8(&raw_src).unwrap();

        let tokens = match string_to_tokens(src) {
            Ok(tokens) => tokens,
            Err(_) => panic!("Unable to parse integer literal"),
        };
        let node = match program(tokens) {
            Ok(node) => node,
            Err(err) => panic!("Failed Parsing: {:?}", err),
        };
        let generated_asm = main(&node);

        let valid_raw =
            fs::read(validation[test_i].clone()).expect("a corresponding validation test");
        let valid = str::from_utf8(&valid_raw).unwrap().to_string();
        println!("{}", valid);
        println!("{}", generated_asm);
        assert_eq!(generated_asm, valid);
    }
}

// Gets all files in a directory, returns them as a vector of path buffers
fn get_all_files(path: &Path) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;
    let all: Vec<PathBuf> = entries
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();
    Ok(all)
}
