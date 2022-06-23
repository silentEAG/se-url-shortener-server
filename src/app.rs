use axum::{Router, routing::{get, post}};

use crate::routers;

#[inline]
pub fn app() -> Router {

    let app = Router::new()
        .route("/", get(routers::default_handler))
        .route("/se", post(routers::url_shorter_handler))
        .route("/:id", get(routers::jumper_handler));
    app
}
