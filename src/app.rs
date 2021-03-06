use axum::{Router, routing::{get, post, options}};

use crate::routers;

#[inline]
pub fn app() -> Router {

    let app = Router::new()
        .route("/", get(routers::default_handler))
        .route("/u", post(routers::url_shorter_handler))
        .route("/u", options(routers::cors_handler))
        .route("/:id", get(routers::jumper_handler));
    app
}
