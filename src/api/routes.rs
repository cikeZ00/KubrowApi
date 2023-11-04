// api/routes.rs

use axum::{routing::get, Router};
use crate::api::handlers::*;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/warframe/ws", get(world_state))
        .route("/warframe/assets/:asset_path", get(assets_handler))
        .route("/test/stinky", get (stinki))
}