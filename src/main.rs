
mod murmur;
mod app;
mod utils;
mod routers;
mod model;
mod config;
mod error;
mod app_type;

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

fn generate_sql_url(sql_config: &SqlConfig) -> String {
    let host = sql_config.host.as_ref().unwrap();
    let port = sql_config.port.as_ref().unwrap();
    let username = sql_config.username.as_ref().unwrap();
    let password = sql_config.password.as_ref().unwrap();
    let database = sql_config.database.as_ref().unwrap();
    let sql_url = format!("mysql://{}:{}@{}:{}/{}", username, password, host, port, database);
    sql_url
}

#[tokio::main]  
async fn main() {

    get_tracing();

    // loading config
    let setted_config = load_config();
    let sql_config = match setted_config.sql_config {
        Some(config) => config,
        None => SqlConfig::default_config()
    };

    // connect to SQL pool
    let pool: Pool<MySql> = MySqlPoolOptions::new()
    .max_connections(match sql_config.max_connections {
        Some(value) => value,
        None => 10
    })
    .connect(generate_sql_url(&sql_config).as_str()).await.unwrap();

    // init server
    let app = app::app()
                    .layer(Extension(AppState {
                        pool,
                        shorter_url_domain: setted_config.app_config.as_ref().unwrap().shorter_url_domain.as_ref().unwrap().to_string(),
                    }));

    tracing::info!("Server starts...");

    let port = setted_config.app_config.as_ref().unwrap().port.as_ref().unwrap();

    let mut addr = String::from("0.0.0.0:");
    addr.push_str(&port);

    // start server
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Server failed!");
}
