#![allow(async_fn_in_trait)]

pub trait Provider {
    type Error;

    async fn update(&mut self) -> Result<(), Self::Error>;
}
