use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;

use crate::models::TodoRepository;

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub async fn todos_index(
    pagination: Option<Query<Pagination>>,
    State(repo): State<Arc<dyn TodoRepository>>,
) -> impl IntoResponse {
    

    let Query(pagination) = pagination.unwrap_or_default();

    match repo.list() {
        Ok(todos) => (StatusCode::OK, Json("todos")),
        Err(_) => (StatusCode::OK, Json("todos")),
    }
    
}


