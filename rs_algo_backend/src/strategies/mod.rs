pub mod general;

use rs_algo_shared::error::Result;
use rs_algo_shared::scanner::instrument::CompactInstrument;

use async_trait::async_trait;
use bson::Document;
use mongodb::Cursor;

#[async_trait(?Send)]
pub trait Strategy {
    // fn default() -> Result<Self>
    // where
    //     Self: Sized;
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn query(&self) -> &Document;
    async fn format_instrument(
        &self,
        instruments: Cursor<CompactInstrument>,
    ) -> Vec<CompactInstrument>;
}
