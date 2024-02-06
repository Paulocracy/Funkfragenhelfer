//! This module contains small but possibly useful
//! Rust utility functions, currently focused on
//! file I/O and time.

// MACRO DIRECTIVES SECTION //
#![allow(dead_code)]

// IMPORTS SECTION //
use rayon::prelude::*;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

// PUBLIC FUNCTIONS SECTION //
/// Ensure the existence of the given directory.
///
/// If the directory exists, nothing is done. If it
/// does not exist, a new directory is created. If an
/// error occurs during the directory creation, the
/// error will be printed.
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

/// Returns the number of seconds since the UNIX epoch as u64.
pub fn get_current_unixtime_in_sec() -> u64 {
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => 0,
    }
}

/// Returns the filenames in the given directory
///
/// You must make sure beforehand that the directory exists. Otherwise,
/// the function panics.
pub fn get_filenames_in_dir(dir: &str) -> Vec<String> {
    let path = Path::new(dir);
    let filenames: Vec<String> = fs::read_dir(path)
        .unwrap()
        .par_bridge()
        .map(|path| path.unwrap().path())
        .filter(|path| path.is_file())
        .map(|path| path.to_str().unwrap().to_owned().replace("\\", "/"))
        .collect();
    filenames
}

/// Returns all lines of the given file as Vec<String>.
///
/// Make sure that the file exists beforehand. Otherwise,
/// this function panics.
pub fn read_filelines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .par_lines()
        .map(String::from)
        .collect()
}

pub fn overwrite_file_lines(filepath: &str, lines: Vec<String>) {
    let path = Path::new(filepath);
    let mut f = fs::File::create(path).unwrap();
    for line in lines {
        writeln!(f, "{}", line).unwrap();
    }
}

pub fn overwrite_file_str(filepath: &str, text: &str) {
    let path = Path::new(filepath);
    let mut f = fs::File::create(path).unwrap();
    writeln!(f, "{}", text).unwrap();
}

pub fn read_filetext(filename: &str) -> String {
    let filelines = read_filelines(filename);
    filelines.join(" ")
}
