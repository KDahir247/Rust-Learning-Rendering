//reminder: need to include documentation when adding new features
/// # Read a specified file within the project directory and parse it into an Option of String.
///
/// To prevent the file stream from reading from my directory and causing errors when others people are trying to use it except me.
/// I've specified the current directory to be the cargo manifest directory (Cargo.toml), so the current directory will be remapped to the
/// user's directory and not mine.
///
/// It start reading from the project directory. To target the specified file you will need to specify the path to file in [file_path] argument.
///
/// return either a String if the file was parsed correctly or a None if the file could not be parsed or could not be located.
///
/// Exception/panic are not allowed and are not thrown to the user if it failed to parse the file.
///
/// <br/>
///
/// ## Ex 1. Basic GLSL file parsing
///
/// ```
/// use util::shader::read_shader;
///
/// let valid_file = read_shader("shader/doc-test/doc-test.vert");
/// assert!(valid_file.is_some());
///
/// let valid_file2 = read_shader("shader/doc-test/doc-test.frag");
/// assert!(valid_file2.is_some());
///
/// ```
///
/// ## Ex 2. Failed to parse file
///
/// ```
/// use util::shader::read_shader;
///
/// let invalid_file = read_shader("some_random_file.vert");
/// assert!(invalid_file.is_none());
///
/// let invalid_file1 = read_shader("some_random.frag");
/// assert!(invalid_file1.is_none());
///
/// ```
pub fn read_shader(file_path: &str) -> Option<String> {
    use std::fs;

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