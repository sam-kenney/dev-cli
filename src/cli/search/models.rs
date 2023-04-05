//! Models for the Search module.
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchResults {
    items: Vec<SearchResult>,
}

/// Implement the `IntoIterator` trait for `SearchResults`.
impl std::iter::IntoIterator for SearchResults {
    type Item = SearchResult;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Convert the `SearchResults` into an iterator.
    /// Iterates over the internal `items` vector.
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

/// A single result from a query.
///
/// # Fields
///
/// * `title` - The title of the result.
/// * `link` - The link to the result.
/// * `snippet` - A description of the result.
#[derive(Deserialize, Debug)]
pub struct SearchResult {
    title: String,
    link: String,
    snippet: String,
}

/// Implement the `print` function for `SearchResult`.
impl SearchResult {
    /// Print a result to the console.
    ///
    /// # Arguments
    ///
    /// * `idx` - The index of the result.
    pub fn print(&self, idx: usize) {
        let description = textwrap::wrap(&self.snippet, 70);
        let description = description.join("\n    ");

        let underline = "-".repeat(self.title.len());

        println!(
            "
\x1b[1m{} | {}\x1b[0m
    {}
    {}

    \x1b[3m{}\x1b[0m
        ",
            idx, self.title, underline, description, self.link
        )
    }
}
