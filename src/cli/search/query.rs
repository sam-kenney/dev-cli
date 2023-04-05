//! Query the Google Custom Search API.
//!
//! Enables searching of Google via your command line.
use crate::cli::download::download_to;
use crate::cli::search::{config::Config, models::SearchResults};

/// Execute a query.
///
/// # Arguments
///
/// * `query` - The query to search for.
/// * `page_num` - The page number to get results for.
pub async fn execute(query: String, page_num: usize) {
    let config = Config::load_from_file().await;

    let results: SearchResults =
        get_results(query, config.api_key, config.search_engine_id, page_num).await;

    for (idx, result) in results.into_iter().enumerate() {
        result.print(idx + 1);
    }
}

/// Get the results for a given query.
///
/// # Arguments
///
/// * `query` - The query to search for.
/// * `api_key` - Your Google API key.
/// * `search_engine_id` - The Google Custom Search Engine ID.
/// * `page_num` - The page number to get results for.
async fn get_results(
    query: String,
    api_key: String,
    search_engine_id: String,
    page_num: usize,
) -> SearchResults {
    let start: usize = page_number(page_num);
    let url = format!(
        "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}&start={}",
        api_key, search_engine_id, query, start
    );

    download_to(url).await.unwrap_or_else(|_| {
        eprintln!("Failed to download results for query: {}", query);
        eprintln!("\nPlease check your API key and search engine ID are correct at");
        eprintln!("{}", Config::dir().to_str().unwrap());
        std::process::exit(1);
    })
}

/// Calulate the result start number for a given page number.
///
/// Works by calculating (page_number - 1) * 10 + 1.
/// Page 1 starts at 1, page 2 starts at 11, etc.
///
/// # Arguments
///
/// * `num` - The page number.
fn page_number(num: usize) -> usize {
    (num - 1) * 10 + 1
}
