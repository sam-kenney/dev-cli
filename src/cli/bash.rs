extern crate alloc;
use alloc::borrow::Cow;
use std::process::{Command, Output};

/// Initialise a new Git repository.
///
/// # Arguments
///
/// * `name` - The name of the project.
pub fn git_init(name: &String) {
    let output: Output = Command::new("git")
        .arg("init")
        .arg(name)
        .output()
        .expect("Failed to execute process");

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_git_init() {
        let name: String = "_test_git_init".to_string();
        git_init(&name);
        let exists: bool = fs::metadata(&name).is_ok();
        assert_eq!(exists, true);
        fs::remove_dir_all(name).unwrap();
    }
}
