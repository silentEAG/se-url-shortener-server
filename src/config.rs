use std::{fs::{File}, io::Read};


use crate::model::conf::Config;

#[inline]
pub fn load_config() -> Config {
    let file_path = "config.toml";
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{:?}", e);
            panic!();
        }
    };

    let mut str_val = String::new();
    file.read_to_string(&mut str_val).expect("Error Reading file.");

    let config: Config = toml::from_str(&str_val).unwrap();

    config
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::mysql::MySqlPoolOptions;

    #[test]
    fn test_load_config() {
        let setted_config = load_config();
        println!("{:?}", setted_config);
    }
    
    #[tokio::test]
    async fn connect_to_sql() {
        let _pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root@localhost/Test").await.unwrap();
    }
}

