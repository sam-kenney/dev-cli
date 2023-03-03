/// Commands for the CLI.
pub mod commands;
/// Execute CLI.
mod exec;
pub use exec::execute;
/// File struct for writing files.
mod file;
/// Language trait & individual lanugage
/// implementations.
mod lang;
/// HTTP request methods.
mod request;
/// Save trait & implementation for `String`.
mod save;
/// Additional utility functions.
mod utils;

pub use utils::get_optional_value;
pub use utils::get_required_value;
pub use utils::get_value_or_default;
