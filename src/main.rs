mod cli;
use clap::{ArgMatches, Command};
use tokio;

#[tokio::main]
async fn main() {
    let matches: ArgMatches = Command::new("dev-cli")
        .version("0.3.0")
        .author("Sam Kenney <sam.kenney@me.com>")
        .about("A CLI for creating development projects")
        .subcommand(cli::commands::project())
        .arg_required_else_help(true)
        .get_matches();

    cli::process_matches(matches).await;
}
