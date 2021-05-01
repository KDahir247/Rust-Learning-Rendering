
/// # Retrieve the absolute path for the specified file relative to the current directory.
///
/// return the absolute path for the specified file if the file exists otherwise it will be `None`.
///
/// Exception/panic are not allowed and are not thrown to the user if it failed to parse the file.
/// <br/>
/// <br/>
/// ## Ex 1. Valid file to get relative absolute path
/// ```
/// use util::core::get_relative_path_file;
///
/// let valid_path = get_relative_path_file("image/Deku.png");
/// assert!(valid_path.is_some());
///
/// assert!(valid_path.unwrap().contains(env!("CARGO_MANIFEST_DIR")));
/// ```
/// ## Ex 2. Invalid file to get relative absolute path
/// ```
///use util::core::get_relative_path_file;
///
/// let invalid_path = get_relative_path_file("image/some_image.png");
/// assert!(invalid_path.is_none());
///
/// ```
/// ```rust,should_panic
/// use util::core::get_relative_path_file;
///
/// let invalid_path2 = get_relative_path_file("image/some_other_image.jpg");
///
/// //Should panic since it is unwrapping a None
/// invalid_path2.unwrap();
///
///
/// ```
pub fn get_relative_path_file(file: &str) -> Option<String>{

    let current_directory = env!("CARGO_MANIFEST_DIR")
        .parse::<String>()
        .expect("parsing failed : CARGO_MANIFEST_DIR failed to parse!!");

    let relative_path = format!("{}/{}", current_directory, file);

    match  std::fs::File::open(&relative_path){
        Ok(_) => { Some(relative_path) }
        Err(_) => { return None; }
    }

}