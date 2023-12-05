use crate::utils::extract_file;
use std::io::BufRead;

pub fn placeholder(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");
}
