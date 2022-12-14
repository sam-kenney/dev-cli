use reqwest;

/// Make a HTTP get request.
///
/// # Arguments
/// * `url` - The url to make a get request to.
///
/// # Returns `reqwest::Response`
/// The `Response` object from the HTTP request.
///
/// # Panics
/// When a response cannot be obtained from a request.
///
/// # Example
/// ```
/// use dev_cli::request;
///
/// let response = request::get("https://www.example.com/");
///
/// match response {
///     Ok(response) => response.text().await
///     Err(e) => panic!("{}", e)
/// }
/// ```
pub async fn get(url: String) -> reqwest::Response {
    let response = reqwest::get(url).await;
    match response {
        Ok(response) => response,
        Err(_) => panic!("Error: Could not get response from request."),
    }
}
