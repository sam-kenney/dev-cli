mod dev_cli;
use clap::{ArgMatches, Command};
use tokio;

/// DevCLI entry point.
///
/// # Subcommands
/// * `project` - Operations for creating and managing projects
#[tokio::main]
async fn main() {
    let matches: ArgMatches = Command::new("dev-cli")
        .version("0.1.0")
        .author("Sam Kenney <sam.kenney@me.com>")
        .about("A CLI for creating development projects")
        .subcommand(dev_cli::commands::project())
        .arg_required_else_help(true)
        .get_matches();

    let cmd: Option<&str> = matches.subcommand_name();

    match cmd {
        Some("project") => {
            let matches: &ArgMatches = matches.subcommand_matches("project").unwrap();
            let cmd: Option<&str> = matches.subcommand_name();

            match cmd {
                Some("new") => {
                    let name: String = dev_cli::get_required_value(matches, "new", "name");

                    let lang: String =
                        dev_cli::get_value_or_default(matches, "new", "lang", "py".to_string());

                    dev_cli::execute(&lang, &name).await
                }
                _ => {}
            }
        }
        _ => {}
    }
}
