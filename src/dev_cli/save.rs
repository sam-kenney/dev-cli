use std::path::Path;

/// Save a file to the filesystem.
///
/// # Examples
/// ```
/// impl Save for String {
///     fn mkdir(&self, dir: &String) -> &Self {
///         let dir: &Path = Path::new(dir);
///         std::fs::create_dir_all(&dir).expect("Unable to create directory.");
///         self
///     }
///
///     fn save(&self, path: &String) -> &Self {
///         let path: &Path = Path::new(path);
///         std::fs::write(&path, self).expect("Unable to write file.");
///         self
///     }
/// }
/// ```
/// # Returns `&Self`
pub trait Save {
    /// Create a directory.
    /// # Arguments
    /// * `dir` - Directory to create
    /// # Returns `&Self`
    fn mkdir(&self, dir: &String) -> &Self;
    /// Save a file.
    /// # Arguments
    /// * `path` - Path to save the file to
    /// # Returns `&Self`
    fn save(&self, path: &String) -> &Self;
}

/// Implement `Save` for `String`.
/// # Examples
///
/// ```
/// use dev_cli::save::Save;
///
/// let dir: String = "test".to_string();
/// let path: String = "test/test.txt".to_string();
/// let text: String = "Hello, world!".to_string();
///
/// text.mkdir(&dir).save(&path);
/// ```
/// # Returns `&Self`
impl Save for String {
    /// Create a directory.
    ///
    /// # Arguments
    /// * `dir` - Directory to create
    /// # Returns `&Self`
    /// # Panics
    /// * If unable to create directory
    fn mkdir(&self, dir: &String) -> &Self {
        let dir: &Path = Path::new(dir);
        std::fs::create_dir_all(&dir).expect("Unable to create directory.");
        self
    }

    /// Save a file.
    /// # Arguments
    /// * `path` - Path to save the file to
    /// # Returns `&Self`
    /// # Panics
    /// * If unable to write file
    fn save(&self, path: &String) -> &Self {
        let path: &Path = Path::new(path);
        std::fs::write(&path, self).expect("Unable to write file.");
        self
    }
}
