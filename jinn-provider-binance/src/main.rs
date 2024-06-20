use jinn_provider::Provider;
use jinn_provider_binance::BinanceProvider;

#[tokio::main]
async fn main() {
    let mut provider = BinanceProvider::new();
    if let Err(e) = provider.update().await {
        eprintln!("Error: {}", e);
    }
    println!("Data: {}", provider.data());
}
