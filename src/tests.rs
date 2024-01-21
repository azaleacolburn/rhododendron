use crate::{code_gen::main, lexer::string_to_tokens, parser::program};
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
    str,
};

#[test]
fn test_all() {
    let tests: Vec<PathBuf> = get_all_files(&Path::new("tests/rh/")).expect("No files to be tests");
    let validation: Vec<PathBuf> =
        get_all_files(&Path::new("tests/validation")).expect("No files for validation");

    assert_eq!(tests.len(), validation.len());
    for test_i in 0..tests.len() {
        let raw_src = fs::read(tests[test_i].clone()).unwrap();
        let src = str::from_utf8(&raw_src).unwrap();
        let names: String = tests[test_i]
            .file_name()
            .unwrap()
            .split(".")
            .collect::<&str>();
        let valid = str::from_utf8(&fs::read(validation[test_i].clone()).unwrap());

        let (tokens, line_tracker) =
            string_to_tokens(src).expect("Unable to parse integer literal");
        let node = program(tokens, line_tracker).unwrap();
        let generated_asm = main(&node);

        Command::new(format!("gcc -o {}.asm {}", names[0], names[0]))
            .spawn()
            .expect("Assembly failed");

        let generated_output = Command::new(format!("./{}", names[0]))
            .output()
            .expect("Failed at runtime");

        assert_eq!(generated_output, valid);
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
