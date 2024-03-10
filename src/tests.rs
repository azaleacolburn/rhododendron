use crate::compiler;
use std::{
    fs, io::{self, BufReader, Read},
    path::{Path, PathBuf},
    process::Command,
    str,
};

#[test]
fn test_all() {
    let tests: Vec<PathBuf> = get_all_files(&Path::new("tests/core")).expect("No files to be tests");

    for test_i in 0..tests.len() {
        let name = tests[test_i].file_stem().unwrap().to_str().unwrap();

        let generated_asm = compiler::test();
        

        Command::new(format!("gcc -o {}.asm {}", name, name))
            .spawn()
            .expect("Assembly failed");

        let generated_output = Command::new(format!("./{}", names))
            .output()
            .expect("Failed at runtime");

        assert_eq!(generated_output);
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
