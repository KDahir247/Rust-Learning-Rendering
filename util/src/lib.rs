use std::fs;
use std::fmt::format;

pub fn read_shader(file_path: &str) -> Option<String> {

    let current_directory = env!("CARGO_MANIFEST_DIR")
        .parse::<String>()
        .expect("parsing failed : CARGO_MANIFEST_DIR failed to parse!!");

    let result =
        fs::read_to_string(format!("{}/{}",current_directory, file_path));

    return match result {
        Ok(file) => { Some(file) }
        Err(_) => {  None }
    }
}