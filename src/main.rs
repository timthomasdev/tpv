use dirs::home_dir;
use reqwest;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio;

#[derive(Deserialize, Debug)]
struct Asset {
    data: Data,
}
#[derive(Deserialize, Debug)]
struct Data {
    symbol: String,
    name: String,
    slug: String,
    market_data: MarketData,
}
#[derive(Deserialize, Debug)]
struct MarketData {
    price_usd: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut porto_sum: f32 = 0.0;

    let mut filename = home_dir().unwrap();
    filename.push(".portfolio");

    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut api_key = String::new();

    reader.read_line(&mut api_key)?;

    for line in reader.lines() {
        if let Ok(holding) = line {
            let mut split = holding.split_whitespace();
            let asset = split.next().unwrap();
            let amount = split.next().unwrap().parse::<f32>().unwrap();

            let api_url = format!(
                "https://data.messari.io/api/v1/assets/{}/metrics/market-data",
                asset
            );
            let res = client
                .get(&api_url)
                .header("x-messari-api-key", "bfe242da-82ab-46f6-abdc-7037a4ad87e7")
                .send()
                .await?
                .json::<Asset>()
                .await?;

            println!(
                "Current price (USD) of {}: {}",
                res.data.name, res.data.market_data.price_usd
            );
            let porto_worth = res.data.market_data.price_usd * amount;
            println!("worth: {}", porto_worth);
            porto_sum += porto_worth;
        }
    }

    println!("Total portfolio: {}", porto_sum);
    Ok(())
}
