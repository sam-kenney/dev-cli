/// Get the current working directory.
/// # Panics
/// Panics if the current working directory cannot be determined.
/// # Examples
/// ```
/// use dev_cli::utils::current_dir;
///
/// let dir: String = current_dir();
/// ```
/// # Returns `String`
/// The current working directory.
pub fn current_dir() -> String {
    match std::env::current_dir() {
        Ok(path) => path.to_str().unwrap_or_else(|| ".").to_string(),
        Err(_) => panic!("Unable to get current directory."),
    }
}
