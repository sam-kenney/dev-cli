use crate::dev_cli::{save::Save, utils};

/// Represents a file to be created.
///
/// # Parameters
/// * `name` - The name of the file.
/// * `dir` - The directory name to save the file in.
/// * `content` - The content of the file.
///
/// # Example
/// ```
/// use dev_cli::file::File;
///
/// let file = File::new("test.txt".to_string(), "test".to_string(), "Hello World!".to_string());
/// file.write();
/// ```
pub struct File {
    pub name: String,
    pub dir: String,
    pub content: String,
}

/// Methods for the File struct.
impl File {
    /// Create a new file.
    ///
    /// # Arguments
    /// * `name` - The name of the file.
    /// * `dir` - The directory name to save the file in.
    /// The directory name will be appended to the current working directory.
    /// * `content` - The content of the file.
    ///
    /// # Returns `Self`
    /// # Example
    /// ```
    /// use dev_cli::file::File;
    ///
    /// let file = File::new("test.txt".to_string(), "test".to_string(), "Hello World!".to_string());
    /// ```
    pub fn new(name: String, dir: String, content: String) -> Self {
        let dir_path: String = format!("{}/{}/", utils::current_dir(), dir);
        Self {
            name: name,
            dir: dir_path,
            content: content,
        }
    }

    /// Write the file to the disk.
    /// # Returns `&Self`
    /// # Example
    /// ```
    /// use dev_cli::file::File;
    ///
    /// let file = File::new("test.txt".to_string(), "test".to_string(), "Hello World!".to_string());
    /// file.write();
    /// ```
    /// # Panics
    /// * If unable to create directory
    pub fn write(&self) -> &Self {
        let file_path: String = format!("{}/{}", self.dir, self.name);
        self.content.to_string().mkdir(&self.dir).save(&file_path);
        self
    }
}
