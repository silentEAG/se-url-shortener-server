use serde::{Deserialize};



#[derive(Deserialize)]
pub struct UrlData {
    pub original_url: String,
}