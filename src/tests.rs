use colored::Colorize;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

#[test]
fn test_all() {
    let tests: Vec<PathBuf> =
        get_all_files(&Path::new("tests/core")).expect("No files to be tests");
    let mut ret = String::new();

    for test_i in 0..tests.len() {
        let name = tests[test_i].file_stem().unwrap().to_str().unwrap();

        println!("name: {}", name);

        Command::new("bash")
            .arg("test_rh.sh")
            .arg(format!("{}", name))
            .spawn()
            .expect("Assembling failed");

        let generated_output = Command::new(format!("./gen/core/{}", name)).output();
        match generated_output {
            Ok(_) => ret.push_str(format!("{}\t\t\t[{}]", name, "OK".green()).as_str()),
            Err(err) => ret.push_str(format!("{}\t\t\t[{}]\n{}", name, "ERR".red(), err).as_str()),
        }
    }

    println!("{}", ret);
}

// Gets all files in a directory, returns them as a vector of path buffers
fn get_all_files(path: &Path) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;
    let all: Vec<PathBuf> = entries
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();
    Ok(all)
}
