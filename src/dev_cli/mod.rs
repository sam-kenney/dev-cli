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
