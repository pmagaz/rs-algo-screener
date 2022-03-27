pub mod stoch;
use rs_algo_shared::error::Result;

use bson::Document;

pub trait Strategy {
    // fn default() -> Result<Self>
    // where
    //     Self: Sized;
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn query(&self) -> &Document;
}
