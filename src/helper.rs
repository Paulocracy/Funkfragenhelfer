#![allow(dead_code)]

use rayon::prelude::*;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn read_filelines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .par_lines()
        .map(String::from)
        .collect()
}

pub fn read_filetext(filename: &str) -> String {
    let filelines = read_filelines(filename);
    filelines.join(" ")
}

pub fn get_filenames_in_dir(dir: &str) -> Vec<String> {
    let path = Path::new(dir);
    let filenames: Vec<String> = fs::read_dir(path)
        .unwrap()
        .par_bridge()
        .map(|path| path.unwrap().path())
        .filter(|path| path.is_file())
        .map(|path| path.to_str().unwrap().to_owned().replace("\\", ""))
        .collect();
    filenames
}

pub fn overwrite_file_str(filepath: &str, text: &str) {
    let path = Path::new(filepath);
    let mut f = fs::File::create(path).unwrap();
    writeln!(f, "{}", text).unwrap();
}

pub fn overwrite_file_lines(filepath: &str, lines: Vec<String>) {
    let path = Path::new(filepath);
    let mut f = fs::File::create(path).unwrap();
    for line in lines {
        writeln!(f, "{}", line).unwrap();
    }
}

pub fn ensure_dir_existence(dir: &str) {
    let path = Path::new(dir);
    if path.exists() && path.is_dir() {
        return;
    }
    match fs::create_dir_all(path) {
        Ok(_) => {}
        Err(e) => {
            println!("Error in creating {}: {}", dir, e)
        }
    }
}