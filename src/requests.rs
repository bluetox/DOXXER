use reqwest;

pub async fn get_request(client: &reqwest::Client, url: &str) -> Result<String, reqwest::Error> {
    let response = match client.get(url).send().await {
        Ok(response) => response,
        Err(err) => {
            eprintln!("Error making request: {}", err);
            return Err(err);
        }
    };


    let data = match response.text().await {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error reading response: {}", err);
            return Err(err);
        }
    };
        Ok(data)
}

pub fn setup_client() -> reqwest::Client {
    let client = reqwest::Client::new();
    client
}