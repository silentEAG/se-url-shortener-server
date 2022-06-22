
mod murmur;
mod app;
mod utils;
mod routers;
mod model;
mod config;

use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, fmt, util::SubscriberInitExt};
use sqlx::{mysql::MySqlPoolOptions, Pool, MySql};
use axum::{Extension};

use crate::{config::load_config, model::{conf::SqlConfig, state::AppState}};

#[inline]
fn get_tracing() {
    tracing_subscriber::registry()
    .with(fmt::layer())
    .init();
}

#[tokio::main]  
async fn main() {

    get_tracing();

    let setted_config = load_config();
    let sql_config = match setted_config.sql_config {
        Some(config) => config,
        None => SqlConfig::default_config()
    };

    let pool: Pool<MySql> = MySqlPoolOptions::new()
    .max_connections(match sql_config.max_connections {
        Some(value) => value,
        None => 5
    })
    .connect("mysql://root:root@localhost/Test").await.unwrap();

    let app = app::app()
                    .layer(Extension(AppState {
                        pool,
                        app_config: setted_config.app_config.unwrap(),
                    }));

    tracing::info!("Server starts...");

    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Server failed!");
}
