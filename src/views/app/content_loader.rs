use std::fs;

#[allow(dead_code)]
pub fn read_file(file_path: &str) -> String {
    let data: String = fs::read_to_string(file_path).expect("Unable to read file");
    return data
}

