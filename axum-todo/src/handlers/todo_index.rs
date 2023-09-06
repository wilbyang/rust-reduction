use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use crate::handlers::internal_error;

use crate::models::{Todo, TodoRepository};

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub async fn todos_index(
    pagination: Option<Query<Pagination>>,
    State(repo): State<Arc<dyn TodoRepository>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Query(pagination) = pagination.unwrap_or_default();

    repo.list().map_err(|e| internal_error(e))
        .and_then(|todos| {
            let offset = pagination.offset.unwrap_or(0);
            let limit = pagination.limit.unwrap_or(100);
            let todos: Vec<Todo> = todos.into_iter().skip(offset).take(limit).collect();
            Ok(Json(todos))
        })
}


