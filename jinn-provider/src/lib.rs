#![allow(async_fn_in_trait)]

use std::borrow::Cow;

pub trait Provider {
    type Error;

    async fn update(&mut self) -> Result<(), Self::Error>;
    fn data(&self) -> Cow<str>;
}
