use serde::{Deserialize};



#[derive(Deserialize)]
pub struct ReqUrlData {
    pub original_url: String,
}


#[derive(Deserialize)]
pub struct ResUrlData {
    pub shorten_url: String,
}