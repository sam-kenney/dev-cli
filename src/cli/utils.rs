use clap::ArgMatches;

/// Access a required parameter value from a subcommand.
///
/// # Arguments
///
/// * `matches` - The matches from the CLI.
/// * `command` - The subcommand to access.
/// * `name` - The name of the parameter to access.
///
/// # Returns
///
/// * `T` - The value of the parameter.
pub fn get_required_value<T>(matches: &ArgMatches, command: &str, name: &str) -> T
where
    T: Clone + Send + Sync + std::str::FromStr + TryInto<T> + 'static,
{
    matches
        .subcommand_matches(command)
        .unwrap()
        .get_one::<String>(name)
        .unwrap_or_else(|| panic!("`{name}` is required"))
        .parse()
        .unwrap_or_else(|_| {
            println!(
                "The value of `{}` must be a valid {}",
                name,
                std::any::type_name::<T>()
            );
            std::process::exit(1);
        })
}

/// Access an optional parameter value from a subcommand.
///
/// # Arguments
///
/// * `matches` - The matches from the CLI.
/// * `command` - The subcommand to access.
/// * `name` - The name of the parameter to access.
///
/// # Returns
///
/// * `Option<T>` - The value of the parameter.
pub fn get_optional_value<T>(matches: &ArgMatches, command: &str, name: &str) -> Option<T>
where
    T: Clone + Send + Sync + std::str::FromStr + TryInto<T> + 'static,
{
    let val: Option<&String> = matches
        .subcommand_matches(command)
        .unwrap()
        .get_one::<String>(name);

    match val {
        Some(v) => {
            let v: T = v.parse().unwrap_or_else(|_| {
                println!(
                    "The value of `{}` must be a valid {}",
                    name,
                    std::any::type_name::<T>()
                );
                std::process::exit(1);
            });
            Some(v)
        }
        None => None,
    }
}

/// Access a parameter value from a subcommand, or return a default value.
///
/// # Arguments
///
/// * `matches` - The matches from the CLI.
/// * `command` - The subcommand to access.
/// * `name` - The name of the parameter to access.
/// * `default` - The default value to return if the parameter is not set.
///
/// # Returns
///
/// * `T` - The value of the parameter, or the default value.
pub fn get_value_or_default<T>(matches: &ArgMatches, command: &str, name: &str, default: T) -> T
where
    T: Clone + Send + Sync + std::str::FromStr + TryInto<T> + 'static,
{
    let val: Option<T> = get_optional_value(matches, command, name);
    match val {
        Some(v) => v,
        None => default,
    }
}

/// Get the current working directory.
/// # Panics
/// Panics if the current working directory cannot be determined.
/// # Examples
/// ```
/// use dev_cli::utils::current_dir;
///
/// let dir: String = current_dir();
/// ```
/// # Returns `String`
/// The current working directory.
pub fn current_dir() -> String {
    match std::env::current_dir() {
        Ok(path) => path.to_str().unwrap_or(".").to_string(),
        Err(_) => panic!("Unable to get current directory."),
    }
}

#[cfg(test)]
mod tests {
    use clap::{Arg, ArgMatches, Command};

    #[test]
    fn test_get_required_value() {
        let matches: ArgMatches = Command::new("dev-cli")
            .subcommand(
                Command::new("test")
                    .about("A test subcommand")
                    .arg(Arg::new("test").required(true)),
            )
            .get_matches_from(vec!["dev-cli", "test", "test"]);

        let val: String = super::get_required_value(&matches, "test", "test");
        assert_eq!(val, "test");
    }

    #[test]
    #[should_panic]
    fn test_get_required_value_panic() {
        let matches: ArgMatches = Command::new("dev-cli")
            .subcommand(Command::new("test").about("A test subcommand"))
            .get_matches_from(vec!["dev-cli", "test"]);

        let _val: String = super::get_required_value(&matches, "test", "test");
    }

    #[test]
    fn test_get_optional_value() {
        let matches: ArgMatches = Command::new("dev-cli")
            .subcommand(
                Command::new("test")
                    .about("A test subcommand")
                    .arg(Arg::new("test").required(true)),
            )
            .get_matches_from(vec!["dev-cli", "test", "test"]);

        let val: Option<String> = super::get_optional_value(&matches, "test", "test");
        assert_eq!(val, Some("test".to_string()));
    }

    #[test]
    fn test_get_value_or_default() {
        let matches: ArgMatches = Command::new("dev-cli")
            .subcommand(
                Command::new("test")
                    .about("A test subcommand")
                    .arg(Arg::new("test")),
            )
            .get_matches_from(vec!["dev-cli", "test", "test"]);

        let val: String =
            super::get_value_or_default(&matches, "test", "test", "default".to_string());
        assert_eq!(val, "test");
    }

    #[test]
    fn test_get_value_or_default_default() {
        let matches: ArgMatches = Command::new("dev-cli")
            .subcommand(
                Command::new("test")
                    .about("A test subcommand")
                    .arg(Arg::new("test")),
            )
            .get_matches_from(vec!["dev-cli", "test"]);

        let val: String =
            super::get_value_or_default(&matches, "test", "test", "default".to_string());
        assert_eq!(val, "default");
    }

    #[test]
    fn test_current_dir() {
        let dir: String = super::current_dir();
        assert_eq!(dir, std::env::current_dir().unwrap().to_str().unwrap());
    }
}
