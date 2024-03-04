use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() {

    let url = std::env::var("URI").unwrap();

    let client = reqwest::Client::new();

    let body = json!({
        "content": "Hello from Rust\nHow are you?"
    });

    let res = client.post(url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await;

    match res {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{:?}", e)
    }
}
