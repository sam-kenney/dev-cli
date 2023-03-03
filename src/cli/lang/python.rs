use crate::cli;

/// Create a Python project.
///
/// # Arguments
///
/// * `name` - The name of the project.
pub async fn python(name: String) {
    println!("Creating Python project `{}`", name);
    let base_url: &str = "https://bitbucket.org/louder/python-template-public/raw/main/";
    let files: Vec<&str> = vec![
        ".editorconfig",
        ".gitignore",
        ".pre-commit-config.yaml",
        "README.md",
        "noxfile.py",
        "requirements-dev.txt",
        "setup.cfg",
    ];

    cli::download::download_files(base_url, files, &name).await;
    make_virtualenv(&name);
    cli::bash::git_init(&name);
}

/// Create a Python virtual environment.
///
/// # Arguments
///
/// * `name` - The name of the project.
fn make_virtualenv(name: &String) {
    use std::process::Command;
    let output = Command::new("python3")
        .arg("-m")
        .arg("venv")
        .arg(format!("{}/venv", name))
        .output()
        .expect("failed to execute process");

    cli::bash::print_cmd_out(output)
}
