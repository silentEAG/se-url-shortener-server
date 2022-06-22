use axum::Json;
use crate::{model::data::UrlData, utils::short_url};


pub async fn url_shorter_handler(Json(frm): Json<UrlData>) -> String {
    let s = frm.original_url.as_str();
    let shorter_url = short_url(s);
    shorter_url
}