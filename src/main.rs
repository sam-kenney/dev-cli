mod cli;
use clap::{ArgMatches, Command};

#[tokio::main]
async fn main() {
    let matches: ArgMatches = Command::new("dev-cli")
        .version("0.3.2")
        .author("Sam Kenney <sam.kenney@me.com>")
        .about("A CLI for creating development projects")
        .subcommand(cli::commands::project())
        .subcommand(cli::commands::search())
        .arg_required_else_help(true)
        .get_matches();

    cli::process_matches(matches).await;
}
