use std::collections::HashMap;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use axum::Json;
use axum::Extension;
use axum::http::HeaderMap;
use axum::http::StatusCode;
use log::debug;
use sqlx::Row;

use crate::app_type::{JsonResponse, Result};
use crate::error::method::*;
use crate::utils::get_cors_header;
use crate::{model::req_struct::ReqUrlData, utils::short_url};
use crate::model::state::AppState;
use crate::app_type::HandlerJsonResult;

enum ExistedState {
    ExistedSame,
    ExistedNotSame,
    NotExisted,
    Error
}

async fn is_Existed(state: &AppState, mur_code: &str, long_url: &str) -> ExistedState {
    let sql = "SELECT id, long_url FROM url_info WHERE is_deleted=false AND mur_hash_code=$1";
    let res = sqlx::query(sql)
    .bind(mur_code)
    .fetch_one(&state.pool).await;
    match res {
        Err(sqlx::Error::RowNotFound) => ExistedState::NotExisted,
        Err(e) => {
            debug!("{}", e);
            ExistedState::Error
        }
        Ok(row) => {
            let db_url = row.get::<String, _>(1);
            if db_url == long_url {
                ExistedState::ExistedSame
            }
            else {
                ExistedState::ExistedNotSame
            }
        }
    }
}

fn generate_reslut_url(domain: String, mur_code: String) -> String {
    let mut result_url = domain;
    if result_url.chars().last() != Some('/') {
        result_url.push('/');
    }
    result_url.push_str(&mur_code);
    result_url
}

fn generate_reslut(shorter_url: String) -> Result<JsonResponse> {
    let mut res = HashMap::new();
    res.insert("shorten_url".to_string(), shorter_url);
    let mut headers = HeaderMap::new();
    headers = get_cors_header(headers);
    return Ok((StatusCode::OK, headers, Json::from(res)));
}

fn valid_length(strs: &str) -> bool {
    strs.len() >= 18_usize && strs.len() < 512_usize
}

// #[axum_macros::debug_handler]
pub async fn url_shorter_handler(
    Extension(state): Extension<AppState>,
    Json(frm): Json<ReqUrlData>
) -> HandlerJsonResult {
    let original_url = frm.original_url.as_str().trim();
    if !valid_length(original_url) {
        return Err(er_unknown());
    }
    let mur_code = short_url(original_url);
    
    match is_Existed(&state, &mur_code, &original_url).await {
        ExistedState::NotExisted => {
            let time = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap().as_secs().to_string();
            let sql = "INSERT INTO url_info(
                long_url, mur_hash_code, insert_at, latest_visit_at, visit_count, is_deleted)
                VALUES($1, $2, $3, $4, 0, false)";
                let res = sqlx::query(&sql)
                .bind(&original_url)
                .bind(&mur_code) 
                .bind(&time)
                .bind(&time)
                .execute(&state.pool).await;
            if let Ok(_) = res {
                let result_url = generate_reslut_url(state.shorter_url_domain.clone(), mur_code);
                return generate_reslut(result_url);
            }
            else {
                if let Err(e) = res {
                    debug!("{}", e);
                }
                return Err(er_insert())
            }
        },
        ExistedState::ExistedSame => {
            let result_url = generate_reslut_url(state.shorter_url_domain.clone(), mur_code);
            return generate_reslut(result_url);
        }
        ExistedState::ExistedNotSame => {
            return Err(er_unknown());
        },
        ExistedState::Error => {
            return Err(er_unknown());
        }
    }
}