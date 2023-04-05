use crate::cli::{self, search::ResultStore};
use clap::ArgMatches;

/// Process the matches from the CLI.
///
/// # Arguments
///
/// * `matches` - The matches from the CLI.
pub async fn process_matches(matches: ArgMatches) {
    let cmd: Option<&str> = matches.subcommand_name();

    match cmd {
        Some("project") => {
            let matches: &ArgMatches = matches.subcommand_matches("project").unwrap();
            let cmd: Option<&str> = matches.subcommand_name();
            process_project_subcommand(matches, cmd).await;
        }

        Some("search") => {
            let last_res: Option<usize> = cli::get_optional_value(&matches, "search", "last");

            if let Some(res) = last_res {
                let url: String = ResultStore::get_link(res).await;
                println!("{}", url);
                std::process::exit(0)
            }

            let query: String = cli::get_required_value(&matches, "search", "query");
            let page_num: usize = cli::get_value_or_default(&matches, "search", "page", 1);
            cli::query::execute(query, page_num).await;
        }

        _ => {}
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
