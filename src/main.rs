mod cli;
use clap::{arg, ArgMatches, Command};
use tokio;

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

    match lang.as_str() {
        "py" => cli::lang::python::execute(name.to_string()).await,
        _ => println!("Language not supported"),
    }
}
