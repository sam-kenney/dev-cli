extern crate alloc;
use alloc::borrow::Cow;
use std::process::{Command, Output};

/// Create a new bash project.
///
/// # Arguments
///
/// * `name` - The name of the project.
pub fn git_init(name: &String) {
    let output: Output = Command::new("git")
        .arg("init")
        .arg(name)
        .output()
        .expect("failed to execute process");

    print_cmd_out(output)
}

/// Print the output of a command.
///
/// # Arguments
///
/// * `output` - The output of a command.
pub fn print_cmd_out(output: Output) {
    let stdout: Cow<str> = String::from_utf8_lossy(&output.stdout);
    let stderr: Cow<str> = String::from_utf8_lossy(&output.stderr);

    if !stdout.is_empty() {
        println!("{}", stdout);
    }
    if !stderr.is_empty() {
        eprintln!("{}", stderr);
    }
}
