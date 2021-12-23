use reqwest::Client;
// cerner_2tothe5th_2021

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fairviewEB = reqwest::get("http://svc.metrotransit.org/NexTrip/902/0/FAUN?format=json")?
    .await?
    .json::<serde_json::Value>()
    .await?;
    println!("{:#?}", resp);
    Ok(());
    let fairviewWB = reqwest::get("http://svc.metrotransit.org/NexTrip/902/1/FAUN?format=json")?
    .await?
    .json::<serde_json::Value>()
    .await?;
    println!("{:#?}", resp);
Ok(());
}
