use std::collections::HashMap;
use std::io;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client};
use serde::Deserialize;
#[derive(Deserialize)]
struct ApiReturn {
    disclaimer: String,
    license: String,
    timestamp: f32,
    base: String,
    rates: HashMap<String, f32>
}
#[tokio::main]
async fn main() {
    let request = request().await;
    println!("{:#?}",request.rates);
}
async fn request() -> ApiReturn {
    let app_id = input();
    let url = format!("https://openexchangerates.org/api/latest.json?app_id={app_id}&base=USD&prettyprint=true&show_alternative=true");
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("accept"), HeaderValue::from_static("application/json"));
    let client = Client::new();
    let response = client.get(url).headers(headers).send()
        .await.unwrap().text().await.unwrap();
    serde_json::from_str(response.as_str()).unwrap()
}
fn input() -> String{
    println!("Do you need to update your exchange rate? Type y for yes, else click enter.");
    let mut update_var = String::new();
    io::stdin().read_line(&mut update_var).unwrap();
    if update_var == "y"{
        //tbd
    }
    let mut app_id = String::new();
    println!("Input your openexchangerates.org app id.");
    io::stdin().read_line(&mut app_id).unwrap();
    app_id
}