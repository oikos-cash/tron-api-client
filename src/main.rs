// https://tronprotocol.github.io/documentation-en/api/http/#fullnode-api
// https://developers.tron.network/reference#wallets-accounts

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
