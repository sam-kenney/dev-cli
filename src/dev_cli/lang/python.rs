use crate::dev_cli::file::File;
use crate::dev_cli::lang::Language;
use crate::dev_cli::request;
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

/// Parse the response from the request.
/// If the response is 200, write the file to the directory.
/// Otherwise, print an error message.
///
/// # Arguments
/// * `response` - The response from the request.
/// * `file` - The file name to write.
/// * `dir` - The directory to write the file to.
///
/// # Example
/// ```
/// use dev_cli::lang::Python;
/// use dev_cli::request;
///
/// let url: String = "https://bitbucket.org/louder/python-template-public/raw/main/README.md".to_string();
/// let response = request::get(url).await;
/// parse_response(response, "README.md".to_string(), ".".to_string()).await;
/// ```
async fn parse_response(response: reqwest::Response, file: String, dir: String) {
    match response.status().as_str() {
        "200" => {
            let content: String = response.text().await.unwrap_or_default();
            let file = File::new(file, dir, content);
            file.write();
            println!("{} created.", file.name)
        },
        _ => println!("Error: Could not get {} from repo.", file),
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
            parse_response(response, file.to_string(), dir.to_string()).await;
        }
    }
}
