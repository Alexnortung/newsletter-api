use reqwest::RequestBuilder;

pub async fn simple_post(
    endpoint: &str,
    email: &str,
    email_field_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post(endpoint).form(&[(email_field_name, email)]);
    let res = res.send().await?;
    if !res.status().is_success() {
        return Err(format!("{}: {}", res.status(), res.text().await?).into());
    }

    Ok(())
}
