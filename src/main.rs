
mod murmur;
mod app;
mod utils;
mod routers;
mod model;
mod config;
mod error;
mod app_type;

use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, fmt, util::SubscriberInitExt};
use axum::{Extension};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

use crate::model::state::AppState;
use crate::config::Config;

#[inline]
fn get_tracing() {
    tracing_subscriber::registry()
    .with(fmt::layer())
    .init();
}

#[inline]
fn get_pg_url(conf: &Config) -> String {
    let pg_url = format!("postgres://{}:{}@{}/{}", conf.pg.user, conf.pg.password, conf.pg.host, conf.pg.dbname);
    println!("{}", pg_url);
    pg_url
}

#[tokio::main]  
async fn main(){

    get_tracing();

    dotenv().ok();

    // loading config
    let conf: Config = Config::from_env().expect("初始化配置失败");
    let pg_url = get_pg_url(&conf);
    let pool = PgPoolOptions::new()
    .max_connections(conf.pg.max_size.parse::<u32>().unwrap())
    .connect(&pg_url).await.unwrap();

    // init server
    let app = app::app()
                    .layer(Extension(AppState {
                        pool,
                        shorter_url_domain: conf.app.url_domain.clone()
                    }));

    tracing::info!("Server starts...");

    let port: String = conf.app.port.clone();

    let mut addr = String::from("0.0.0.0:");
    addr.push_str(&port);

    // start server
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Server failed!");
}
