use std::collections::HashMap;
use std::io;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use dirs::home_dir;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_vec};

#[derive(Deserialize, Serialize)]
struct ApiReturn {
    disclaimer: String,
    license: String,
    timestamp: f32,
    base: String,
    rates: HashMap<String, f32>
}
#[tokio::main]
async fn main() {
    println!("Do you need to update your exchange rates? Type y for yes, else click enter.");
    let mut update_var = String::new();
    io::stdin().read_line(&mut update_var).unwrap();
    if update_var == "y"{
        save_currencies(request().await);
    }
    let request = request().await;
    println!("{:#?}",request.rates);
}
async fn request() -> ApiReturn {
    let app_id = app_id();
    let url = format!("https://openexchangerates.org/api/latest.json?app_id={app_id}&base=USD&prettyprint=true&show_alternative=true");
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("accept"), HeaderValue::from_static("application/json"));
    let client = Client::new();
    let response = client.get(url).headers(headers).send()
        .await.unwrap().text().await.unwrap();
    serde_json::from_str(response.as_str()).unwrap()
}
fn app_id() -> String{
    let app_id_string = fs::read_to_string(config_file().as_mut_os_string().append("app_id"));
    match app_id_string {
        Ok(app_id_string) => app_id_string,
        _Err => {
            let mut app_id = String::new();
            println!("Input your openexchangerates.org app id.");
            io::stdin().read_line(&mut app_id).unwrap();
            let mut file = File::create(config_file()).unwrap();
            file.write(app_id.as_bytes()).unwrap();
            app_id
        }
    }
}
fn save_currencies(save_json: ApiReturn){
    fs::write(
        config_file().push("currency.json").as_path(),
        json![to_vec(&save_json)]
    )
        .expect("Should create or overwrite file.");
}
fn config_file() -> PathBuf {
    PathBuf::from(
        &[
            home_dir()
                .unwrap_or_else(|| "".into())
                .display()
                .to_string(),
            "/Documents/".to_string(),
        ]
            .join(""),
    )
}