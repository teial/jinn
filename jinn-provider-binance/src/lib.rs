mod error;

use jinn_provider::Provider;
use log::info;
use ureq::Agent;

pub struct BinanceProvider {
    client: Agent,
    data: String,
}

impl BinanceProvider {
    pub fn new() -> Self {
        BinanceProvider {
            client: Agent::new(),
            data: String::new(),
        }
    }
}

impl Provider for BinanceProvider {
    type Error = error::Error;

    async fn update(&mut self) -> Result<(), Self::Error> {
        self.data = self
            .client
            .get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT")
            .call()
            .map_err(|_| error::Error::UpdateError)?
            .into_string()
            .map_err(|_| error::Error::UpdateError)?;
        info!("Received data: {}", self.data);
        Ok(())
    }
}
