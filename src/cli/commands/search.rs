use clap::{arg, Arg, Command};

pub fn search() -> Command {
    Command::new("search")
        .about("Search Google with a query")
        .arg(Arg::new("query").required(false))
        .arg(arg!(-p --page <usize> "Page number to return"))
        .arg(arg!(-l --last <usize> "Last result index to get the url for"))
        .arg_required_else_help(true)
}
