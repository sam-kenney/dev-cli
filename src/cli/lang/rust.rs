use crate::cli;
use std::process::{Command, Output};

/// Create a Rust project.
///
/// # Arguments
///
/// * `name` - The name of the project.
pub fn rust(name: String) {
    println!("Creating Rust project `{}`", name);
    let output: Output = Command::new("cargo")
        .arg("new")
        .arg(name)
        .output()
        .expect("Failed to execute process");

    cli::bash::print_cmd_out(output)
}
