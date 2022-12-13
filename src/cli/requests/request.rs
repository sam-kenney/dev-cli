use reqwest;

pub async fn get(url: String) -> String {
    let response = reqwest::get(url).await;
    match response {
        Ok(response) => {
            let content = response.text().await;
            match content {
                Ok(content) => content,
                Err(_) => panic!("Error: Could not get content from response."),
            }
        }
        Err(_) => panic!("Error: Could not get response from request."),
    }
}
