use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use axum::Json;
use axum::Extension;
use sqlx::Row;

use crate::{model::req_res_struct::UrlData, utils::short_url};
use crate::model::state::AppState;


pub async fn url_shorter_handler(
    Extension(state): Extension<AppState>,
    Json(frm): Json<UrlData>) -> String {
    let original_url = frm.original_url.as_str();
    let shorter_url = short_url(original_url);
    println!("{:?}", shorter_url);
    let sql = "SELECT `id`, `long_url` FROM url_shorter.url_info WHERE `is_deleted`=0 AND `mur_hash_code`=(?)";
    let conn = sqlx::query(sql)
            .bind(&shorter_url.as_str())
            .fetch_one(&state.pool).await;
    
    if let Err(sqlx::Error::RowNotFound) = conn {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap().as_secs().to_string();
        let sql = format!("INSERT INTO url_shorter.url_info(
            long_url, mur_hash_code, insert_at, latest_visit_at, visit_count, is_deleted)
            VALUES('{}', '{}', '{}', '{}', 0, 0)"
            , original_url, shorter_url, time, time);
        println!("{}", sql);
        let _ = sqlx::query(&sql)
            .execute(&state.pool).await.unwrap();
    }
    else {
        let long_url = conn.unwrap().try_get::<String, _>(1).unwrap();
        if original_url != long_url {
            return "Unimplemented!".to_string();
        }
    }
    // println!("{:?}", conn.try_get::<String, _>(1));

    let mut result_url = state.shorter_url_domain.clone(); 
    if result_url.chars().last() != Some('/') {
        result_url.push('/');
    }
    result_url.push_str(&shorter_url);
    result_url
}