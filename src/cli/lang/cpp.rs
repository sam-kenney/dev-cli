use crate::cli;

/// Create a C++ project.
///
/// # Arguments
///
/// * `name` - The name of the project.
pub async fn cpp(name: String) {
    println!("Creating C++ project `{}`", name);
    let base_url: &str = "https://raw.githubusercontent.com/sam-kenney/cpp-template/main/";
    let files: Vec<&str> = vec![
        "src/Main.cpp",
        "src/Calculate.cpp",
        "src/Calculate.hpp",
        "tests/Main.cpp",
        "tests/Test.cpp",
        "tests/Test.hpp",
        ".editorconfig",
        ".gitignore",
        "Makefile",
    ];

    cli::download::download_files(base_url, files, &name).await;
    cli::bash::git_init(&name);
}
