use crate::dev_cli::lang::{Language, Python};

/// Run the CLI.
///
/// # Arguments
/// * `lang` - The language provided via the cli args.
/// * `name` - The project name. Used as the dir name.
///
/// # Example
/// ```
/// use dev_cli;
///
/// dev_cli::execute("py".to_string(), "new-proj".to_string());
/// ```
pub async fn execute(lang: &String, name: &String) {
    match lang.as_str() {
        "py" => Python::new().execute(&name).await,
        _ => eprintln!("Language `{}` not yet supported", lang),
    }
    std::process::exit(1)
}
