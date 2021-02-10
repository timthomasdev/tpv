use reqwest;
use serde::Deserialize;
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
async fn main() -> Result<(), reqwest::Error> {
    let mut porto_sum: f32 = 0.0;

    let holdings = [
        ("xtz", 5950.0),
        ("ltc", 3.4102915),
        ("eth", 0.97343448),
        ("btc", 0.03997992),
    ];
    let client = reqwest::Client::new();

    for (asset, amount) in holdings.iter() {
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

    println!("Total portfolio: {}", porto_sum);
    Ok(())
}
