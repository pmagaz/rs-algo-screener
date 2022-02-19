use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Instrument {
    pub grant_type: String,
    pub access_code: String,
    pub redirect_url: String,
}
