use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct Ip{
    ip:String,
    hostname: String,
    city: String,
    region: String,
    country:String,
    loc: String,
    postal: String,
    timezone:String,
    readme:String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let req:Ip = reqwest::Client::new()
    .get("https://ipinfo.io/json")
    .send()
    .await?
    .json()
    .await?;

    println!("Seu ip é: \x1b[1m\x1b[92m{}\x1b[0m", req.ip);
    println!("Seu time zone é: \x1b[1m\x1b[92m{}\x1b[0m", req.timezone);
    println!("Sua cidade é: \x1b[1m\x1b[92m{}\x1b[0m", req.city);
    println!("Seu estádo é: \x1b[1m\x1b[92m{}\x1b[0m", req.region);
    println!("Seu paíz é: \x1b[1m\x1b[92m{}\x1b[0m", req.country);
    
    Ok(())
}
