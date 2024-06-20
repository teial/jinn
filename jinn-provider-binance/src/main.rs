use anyhow::Result;
use jinn_provider::Provider;
use jinn_provider_binance::BinanceProvider;

#[tokio::main]
async fn main() -> Result<()> {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();

    let mut provider = BinanceProvider::new();
    provider.update().await?;

    Ok(())
}
