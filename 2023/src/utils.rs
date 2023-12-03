use std::fs::File;
use std::io::{BufReader, Error};

use std::path::Path;

pub fn extract_file(file_path: &String) -> Result<BufReader<File>, Error> {
    let path = Path::new(file_path);
    let display = path.display();

    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening the file: {}", e);
            eprintln!("Fpath: {}", display);
            return Err(e);
        }
    };

    let reader = BufReader::new(file);

    Ok(reader)
}
