

pub async fn  api_get_request(url: &str) -> String {

    let body = reqwest::get(url).await.expect("unable to call api")
    .text().await.expect("unable to get text");

    return body
}
