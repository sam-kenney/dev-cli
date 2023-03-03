use crate::dev_cli::file::File;
use clap::ArgMatches;
use reqwest::Response;

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
/// * `&T` - The value of the parameter.
pub fn get_required_value<T>(matches: &ArgMatches, command: &str, name: &str) -> T
where
    T: Clone + Send + Sync + std::str::FromStr + TryInto<T> + 'static,
{
    matches
        .subcommand_matches(command)
        .unwrap()
        .get_one::<String>(name)
        .expect(format!("`{}` is required", name).as_str())
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
/// * `Option<&T>` - The value of the parameter.
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
        Ok(path) => path.to_str().unwrap_or_else(|| ".").to_string(),
        Err(_) => panic!("Unable to get current directory."),
    }
}

/// Parse the response from the request.
///
/// If the response is 200, write the file to the directory.
/// Otherwise, print an error message.
///
/// # Arguments
/// * `response` - The response from the request.
/// * `file` - The file name to write.
/// * `dir` - The directory to write the file to.
///
/// # Example
/// ```
/// use dev_cli::request;
///
/// let url: String = "https://bitbucket.org/louder/python-template-public/raw/main/README.md".to_string();
/// let response = request::get(url).await;
/// parse_response(response, "README.md".to_string(), ".".to_string()).await;
/// ```
pub async fn write_file_from_response(response: Response, file: String, dir: String) {
    match response.status().as_str() {
        "200" => {
            let content: String = response.text().await.unwrap_or_default();
            let file = File::new(file, dir, content);
            file.write();
            println!("{} created.", file.name)
        }
        _ => println!("Error: Could not get {} from repo.", file),
    }
}
