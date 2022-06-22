use axum::Extension;

use crate::model::state::AppState;



pub async fn default_handler(
    Extension(state): Extension<AppState>,
) -> String {
    let s = state.app_config.domain.unwrap();
    // String::from("Hello SilentE!")
    s
}