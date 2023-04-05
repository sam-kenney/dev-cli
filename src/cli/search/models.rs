//! Models for the Search module.
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
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
#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug)]
pub struct ResultStoreItem {
    index: usize,
    link: String,
}

impl ResultStoreItem {
    pub fn new(index: usize, link: String) -> Self {
        Self { index, link }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResultStore {
    items: Vec<ResultStoreItem>,
}

impl ResultStore {
    pub async fn from_search_results(results: SearchResults) -> Self {
        let mut items = Vec::new();

        for (index, result) in results.into_iter().enumerate() {
            let index = index + 1;
            let item = ResultStoreItem::new(index, result.link);
            items.push(item);
        }

        Self { items }
    }

    pub fn dir() -> PathBuf {
        let path = dirs::home_dir().unwrap();
        path.join(".dev-config/results.yaml")
    }

    pub async fn write(&self) {
        let path = Self::dir();
        if !path.parent().unwrap().exists() {
            tokio::fs::create_dir_all(&path.parent().unwrap())
                .await
                .expect("Failed to create directory `.dev-config`");
        }

        tokio::fs::write(&path, serde_yaml::to_string(&self).unwrap())
            .await
            .expect("Failed to write to file");
    }

    pub async fn get_link(index: usize) -> String {
        let path = Self::dir();

        if !path.exists() {
            return String::new();
        }

        let contents = tokio::fs::read_to_string(path)
            .await
            .expect("Failed to read file");
        let store: ResultStore = serde_yaml::from_str(&contents).expect("Failed to parse YAML");

        let item = store
            .items
            .iter()
            .find(|item| item.index == index)
            .unwrap_or_else(|| panic!("No item with index {} found", index));

        item.link.clone()
    }
}
