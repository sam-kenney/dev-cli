use clap::{arg, Arg, Command};

/// Operations for creating and managing projects.
///
/// # Subcommands
/// * `new` - Create a new project
pub fn project() -> Command {
    Command::new("project")
        .about("Operations for creating and managing projects")
        .subcommand(new())
        .arg_required_else_help(true)
}

/// Create a new project.
///
/// # Arguments
/// * `name` - Name of the project
/// * `-l --lang <String>` - Language to generate a project for
fn new() -> Command {
    Command::new("new")
        .about("Create a new project")
        .arg(Arg::new("name").required(true))
        .arg(arg!(-l --lang <String> "Language to generate a project for"))
        .arg_required_else_help(true)
}
