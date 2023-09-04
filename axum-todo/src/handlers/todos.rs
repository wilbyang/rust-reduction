use axum::extract::{Query, State};
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use crate::models::Db;

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub async fn todos_index(
    pagination: Option<Query<Pagination>>,
    State(db): State<Db>,
) -> impl IntoResponse {
    

    let Query(pagination) = pagination.unwrap_or_default();

    let todos = db.into_read_only().values()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect::<Vec<_>>();

    Json(todos)
}