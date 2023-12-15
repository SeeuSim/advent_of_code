use crate::utils::extract_file;


pub fn placeholder_two(file_name: &String) {
    let _reader = extract_file(file_name).expect("An error occurred while reading the file");
}
