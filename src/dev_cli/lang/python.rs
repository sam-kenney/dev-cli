use crate::dev_cli::{lang::Language, request, utils};
use async_trait::async_trait;

/// Python project struct.
///
/// # Arguments
/// * `base_url` - The url to the repository to get files from.
/// * `files` - A vec of file names to get from the repository.
///
/// # Example
/// ```
/// use dev_cli::lang::Python;
///
/// Python::new().execute();
/// ```
pub struct Python {
    base_url: String,
    files: Vec<String>,
}

/// Implement methods on the Python struct.
impl Python {
    /// Create a new Python struct.
    ///
    /// Uses default URL for Louder's Bitbucket repository
    /// for a Python template.
    ///
    /// # Returns `Python`
    pub fn new() -> Python {
        Python {
            base_url: "https://bitbucket.org/louder/python-template-public/raw/main/".to_string(),
            files: vec![
                ".editorconfig",
                ".gitignore",
                ".pre-commit-config.yaml",
                "README.md",
                "noxfile.py",
                "requirements-dev.txt",
                "setup.cfg",
            ]
            .iter()
            .map(|&s| s.to_string())
            .collect(),
        }
    }
}

/// Implement the Language trait for the Python struct.
#[async_trait]
impl Language for Python {
    /// Create a new Python project.
    ///
    /// # Arguments
    /// * `dir` - The directory to write the files to.
    async fn execute(&self, dir: &String) {
        for file in self.files.iter() {
            let url: String = format!("{}{}", self.base_url, file);
            let response: reqwest::Response = request::get(url).await;
            utils::write_file_from_response(response, file.to_string(), dir.to_string()).await;
        }
    }
}
