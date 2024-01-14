use reqwest;
use serde_json::{Value};

#[derive(Debug)]
pub struct DataVariation {
    pub date: String,
    pub percentage_variation: i64
}

pub async fn fetch_data_and_return_percentage_variations(month: u32) -> Result<Vec<DataVariation>, Box<dyn std::error::Error>> {
    let base_url = "https://api.currencyapi.com/v3/historical";
    let api_key = "cur_live_dAi66m8hGjlT9ndFmeX6aXgnYZzDkH8nbrH34He0";
    let base_currency = "BTC";
    let currency = "USD";

    let mut percentage_variations = Vec::new();

    let last_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 28,
        _ => return Ok(percentage_variations),
    };

    println!("Fetching data for month {}", last_day);

    let mut previous_value: Option<f64> = None;

    for day in 14..=25 {
        let mut date = format!("2023-{:02}-{:02}", month, day);

        let url = format!(
            "{}?apikey={}&currencies={}&base_currency={}&date={}",
            base_url, api_key, currency, base_currency, date
        );

        let client = reqwest::Client::new();
        let response = client.get(&url).query(&[("context", "hello")])
            .header("Content-Type", "application/json")
            .header("X-API-Key", api_key)
            .send()
            .await?;
        let body = response.text().await?;
        let json_response: Value = serde_json::from_str(&body)?;

        if let Some(current_value) = json_response["data"][currency]["value"].as_f64() {
            if let Some(prev_value) = previous_value {
                let percentage_variation = ((current_value - prev_value) / prev_value * 100.0).round() as i64;
                date = format!("\"2023-{:02}-{:02}\"", month, day);
                percentage_variations.push(DataVariation { date, percentage_variation });
            }
            previous_value = Some(current_value);
        }
    }

    Ok(percentage_variations)
}