use axum::{
    routing::{get, post},
    Router, Json,
};
use log::{info};
mod murmur;
mod utils;
use serde::Deserialize;
use utils::short_url;
#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(root))
        .route("/se", post(create_url_short_test));
    info!("Service start...");

    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> String {
    String::from("Hello SilentE!")
}

#[derive(Deserialize)]
pub struct UrlData {
    pub original_url: String,
}

async fn create_url_short_test(Json(frm): Json<UrlData>) -> String {
    let s = frm.original_url.as_str();
    let shorter_url = short_url(s);
    shorter_url
}