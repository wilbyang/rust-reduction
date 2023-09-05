use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::models::TodoRepository;

pub async fn todos_delete(Path(id): Path<Uuid>, State(repo): State<Arc<dyn TodoRepository>>) -> impl IntoResponse {
    match repo.delete(id) {
        Ok(_) => (StatusCode::NO_CONTENT, Json("")),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("error")),
    }
}