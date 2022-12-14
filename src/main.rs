mod dev_cli;
use clap::{arg, ArgMatches, Command};
use tokio;

/// DevCLI entry point.
///
/// Create a new project for a given language.
///
/// # Arguments
/// * `--lang` - Language to generate a project for
/// * `--name` - Name to give the project
#[tokio::main]
async fn main() {
    let matches: ArgMatches = Command::new("dev-cli")
        .version("0.1.0")
        .author("Sam Kenney <sam.kenney@me.com>")
        .about("A CLI for creating development projects")
        .arg(arg!(--lang <String> "Language to generate a project for").required(true))
        .arg(arg!(--name <String> "Name to give the project").required(true))
        .get_matches();

    let lang: &String = matches.get_one::<String>("lang").expect("required");
    let name: &String = matches.get_one::<String>("name").expect("required");

    dev_cli::execute(lang, name).await
}
