

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub port: String,
    pub url_domain: String
}

#[derive(Deserialize)]
pub struct PGConfig {
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub port: String,
    pub host: String,
    pub max_size: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub pg: PGConfig,
}

impl Config {
    /// 从环境变量中初始化配置
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}

#[tokio::test]
async fn connect_to_sql() {
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    dotenv().ok();
    let conf = Config::from_env().expect("初始化配置失败");
    let pg_url = format!("postgres://{}:{}@{}/{}", conf.pg.user, conf.pg.password, conf.pg.host, conf.pg.dbname);
    println!("{}", pg_url);
    let pool = PgPoolOptions::new()
    .max_connections(conf.pg.max_size.parse::<u32>().unwrap())
    .connect(&pg_url).await;
}


