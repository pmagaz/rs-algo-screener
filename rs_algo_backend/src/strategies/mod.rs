pub mod general;

use rs_algo_shared::error::Result;
use rs_algo_shared::scanner::instrument::CompactInstrument;

use bson::Document;
use mongodb::Cursor;

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
