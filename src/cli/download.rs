use std::fs::{self, File};
use std::path::Path;

/// Download files from a URL.
///
/// # Arguments
///
/// * `base_url` - The base URL to download the files from.
/// * `files` - The files to download.
/// * `name` - The name of the project.
pub async fn download_files(base_url: &str, files: Vec<&str>, name: &String) {
    use crate::cli::utils;
    let dir: String = format!("{}/{}", utils::current_dir(), name);

    mkdir_if_not_exists(dir).await;

    let tasks = files.iter().map(|file| {
        let url = format!("{}{}", base_url, file);
        let path = format!("{}/{}/{}", utils::current_dir(), name, file);
        download_file(url, path)
    });

    futures::future::join_all(tasks).await;
}

/// Download a file from a URL.
///
/// # Arguments
///
/// * `url` - The URL to download the file from.
/// * `path` - The path to save the file to.
async fn download_file(url: String, path: String) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    fs::create_dir_all(Path::new(&path).parent().unwrap()).unwrap();
    let mut out = File::create(path)?;
    std::io::copy(&mut resp.as_bytes(), &mut out)?;
    Ok(())
}

/// Download text from a URL.
///
/// # Arguments
///
/// * `url` - The URL to download the text from.
pub async fn download_to<T>(url: String) -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned,
{
    let resp = reqwest::get(url).await?.json::<T>().await?;
    Ok(resp)
}

/// Create a directory if it does not exist.
///
/// # Arguments
///
/// * `dir` - The directory to create.
async fn mkdir_if_not_exists(dir: String) {
    if !Path::new(&dir).exists() {
        tokio::fs::create_dir(dir).await.unwrap();
        return;
    }
    eprintln!("Directory {} already exists", dir);
    std::process::exit(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[tokio::test]
    async fn test_download_file() {
        let url: String =
            "https://raw.githubusercontent.com/sam-kenney/dev-cli/main/.gitignore".to_string();
        let path: String = format!(
            "{}/_test_download_file/.gitignore",
            std::env::current_dir().unwrap().to_str().unwrap()
        );
        download_file(url, path).await.unwrap();
        let mut file = File::open("_test_download_file/.gitignore").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "/target\n");
        fs::remove_dir_all("_test_download_file").unwrap();
    }

    #[tokio::test]
    async fn test_mkdir_if_not_exists() {
        let dir: String = format!(
            "{}/_test_mkdir_if_not_exists",
            std::env::current_dir().unwrap().to_str().unwrap()
        );
        mkdir_if_not_exists(dir.clone()).await;
        assert!(Path::new(&dir).exists());
        fs::remove_dir(dir).unwrap();
    }

    #[tokio::test]
    async fn test_download_files() {
        let base_url: String =
            "https://raw.githubusercontent.com/sam-kenney/dev-cli/main/".to_string();
        let files: Vec<&str> = vec![".gitignore"];
        let name: String = "_test_download_files".to_string();
        download_files(&base_url, files, &name).await;
        let mut file = File::open("_test_download_files/.gitignore").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "/target\n");
        fs::remove_dir_all("_test_download_files").unwrap();
    }
}
