use crate::cli::requests::request;
use crate::cli::file::File;

const BASE_URL: &str = "https://bitbucket.org/louder/python-template-public/raw/main/";

const FILES: [&str; 7] = [
    ".editorconfig",
    ".gitignore",
    ".pre-commit-config.yaml",
    "README.md",
    "noxfile.py",
    "requirements-dev.txt",
    "setup.cfg",
];

fn request_url(file: String) -> String {
    format!("{}{}", BASE_URL, file)
}

pub async fn execute(dir: String) {
    for file in FILES.iter() {
        let url = request_url(file.to_string());
        let content = request::get(url).await;
        let f = File::new(file.to_string(), dir.to_string(), content);
        f.write();
        println!("{} created.", file);
    }
}
