use clap::{arg, Arg, Command};

pub fn search() -> Command {
    Command::new("search")
        .about("Search Google with a query")
        .arg(Arg::new("query").required(true))
        .arg(arg!(-p --page <usize> "Page number to return"))
        .arg_required_else_help(true)
}
