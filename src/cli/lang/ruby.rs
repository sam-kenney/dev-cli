use crate::cli;

/// Create a Ruby project.
///
/// # Arguments
///
/// * `name` - The name of the project.
pub async fn ruby(name: String) {
    println!("Creating Ruby project `{}`", name);
    let base_url = "https://raw.githubusercontent.com/sam-kenney/ruby-template/main/";
    let files: Vec<&str> = vec![
        "spec/calculate_spec.rb",
        "src/calculate.rb",
        "src/main.rb",
        ".gitignore",
        ".rubocop.yml",
        ".ruby-version",
        "Gemfile",
        "Gemfile.lock",
        "Rakefile",
    ];

    cli::download::download_files(base_url, files, &name).await;
    cli::bash::git_init(&name);
}
