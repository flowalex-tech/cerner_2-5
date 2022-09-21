use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;
// Learning Rust requests better https://dev.to/hb/how-to-fetch-a-web-api-with-rust-1390
// cerner_2tothe5th_2022
#[derive(Serialize, Deserialize, Debug)]
struct CompanyQuote {
    c: f64,
    h: f64,
    l: f64,
    o: f64,
    pc: f64,
    t: i128,
}

impl CompanyQuote {
    async fn get(symbol: &String, api_key: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<CompanyQuote>().await?;

        Ok(res)
    }
}
