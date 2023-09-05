use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use uuid::Uuid;
use crate::models::{Todo, TodoRepository};

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

pub async fn todos_update(
    Path(id): Path<Uuid>,
    State(repo): State<Arc<dyn TodoRepository>>,
    Json(input): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {

    Ok(Json("todo"))
}