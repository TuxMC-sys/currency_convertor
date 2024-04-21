use std::collections::HashMap;
use std::{io, io::Write, fs, fs::File};
use std::path::PathBuf;
use dirs::home_dir;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::to_vec;
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
    if update_var == "y\n"{
        save_currencies(request().await);
    }
    convert_currencies(load_currencies().rates);
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
    let mut path = config_file().into_os_string();
    path.push("app_id");
    let app_id_string = fs::read_to_string(path.as_os_str());
    match app_id_string {
        Ok(app_id_string) => app_id_string,
        _err => {
            let mut app_id = String::new();
            println!("Input your openexchangerates.org app id");
            io::stdin().read_line(&mut app_id).unwrap();
            let mut file = File::create(path).unwrap();
            file.write(app_id.as_bytes()).unwrap();
            app_id
        }
    }
}
fn save_currencies(save_json: ApiReturn){
    let mut path = config_file().into_os_string();
    path.push("currency.json");
    fs::write(path.as_os_str(), to_vec(&save_json).unwrap())
        .expect("Should create or overwrite file.");
}
fn load_currencies() -> ApiReturn{
    let mut path = config_file().into_os_string();
    path.push("currency.json");
    serde_json::from_slice(&*fs::read(path).expect("Re-run and refresh your currencies.")).unwrap()
}
fn convert_currencies(currency_map: HashMap<String, f32>){
    println!("What currency do you want to convert from (i.e. USD, GBP, or EUR?");
    let mut orgin_currency = String::new();
    io::stdin().read_line(&mut orgin_currency).unwrap();
    orgin_currency.truncate(orgin_currency.len() - 1);
    println!("What currency do you want to convert to?");
    let mut final_currency = String::new();
    io::stdin().read_line(&mut final_currency).unwrap();
    final_currency.truncate(final_currency.len() - 1);
    println!("How much {} do you want to know the value of in {}?", orgin_currency, final_currency);
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).unwrap();
    let orgin_multiplier = currency_map.get(&orgin_currency).expect("Invalid currency name");
    let final_multiplier = currency_map.get(&final_currency).expect("Invalid currency name");
    amount.truncate(amount.len()-1);
    let amount = amount.parse::<f32>().unwrap();
    let final_amount = orgin_multiplier * final_multiplier * amount;
    println!("{} {} is {} {}",amount, orgin_currency, final_amount, final_currency)

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