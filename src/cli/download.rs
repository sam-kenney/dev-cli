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
