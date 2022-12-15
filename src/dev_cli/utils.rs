use crate::dev_cli::file::File;
use reqwest::Response;

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

/// Parse the response from the request.
///
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
/// use dev_cli::request;
///
/// let url: String = "https://bitbucket.org/louder/python-template-public/raw/main/README.md".to_string();
/// let response = request::get(url).await;
/// parse_response(response, "README.md".to_string(), ".".to_string()).await;
/// ```
pub async fn write_file_from_response(response: Response, file: String, dir: String) {
    match response.status().as_str() {
        "200" => {
            let content: String = response.text().await.unwrap_or_default();
            let file = File::new(file, dir, content);
            file.write();
            println!("{} created.", file.name)
        }
        _ => println!("Error: Could not get {} from repo.", file),
    }
}
