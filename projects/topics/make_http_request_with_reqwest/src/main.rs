use reqwest::Error;

async fn fetch(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://www.example.com";
    let response_body = fetch(url).await?;
    println!("{}", response_body);
    Ok(())
}
