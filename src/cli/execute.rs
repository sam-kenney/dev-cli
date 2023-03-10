use crate::cli::lang;

/// Execute the project.
pub async fn execute(name: String, lang: String) {
    match lang.as_str() {
        "py" => {
            lang::python(name).await;
            std::process::exit(0);
        }
        "rb" => {
            lang::ruby(name).await;
            std::process::exit(0);
        }
        "rs" => {
            lang::rust(name);
            std::process::exit(0);
        }
        _ => {
            eprintln!("Language `{}` not yet supported", lang);
            std::process::exit(1);
        }
    }
}
