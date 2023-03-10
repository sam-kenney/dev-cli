use crate::cli;
use clap::ArgMatches;

/// Process the matches from the CLI.
///
/// # Arguments
///
/// * `matches` - The matches from the CLI.
pub async fn process_matches(matches: ArgMatches) {
    let cmd: Option<&str> = matches.subcommand_name();

    if let Some("project") = cmd {
        let matches: &ArgMatches = matches.subcommand_matches("project").unwrap();
        let cmd: Option<&str> = matches.subcommand_name();
        process_project_subcommand(matches, cmd).await;
    }
}

/// Process the matches from the `project` subcommand.
///
/// # Arguments
///
/// * `matches` - The matches from the `project` subcommand.
/// * `cmd` - The subcommand of the `project` subcommand.
async fn process_project_subcommand(matches: &ArgMatches, cmd: Option<&str>) {
    if let Some("new") = cmd {
        let name: String = cli::get_required_value(matches, "new", "name");
        let lang: String = cli::get_value_or_default(matches, "new", "lang", "py".to_string());
        cli::execute(name, lang).await
    }
}
