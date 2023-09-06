use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use uuid::Uuid;
use crate::models::{Todo, TodoRepository};

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    text: String,
}

pub async fn todos_create(State(repo): State<Arc<dyn TodoRepository>>, Json(input): Json<CreateTodo>) -> impl IntoResponse {
    let todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed: false,
    };

    repo.save(todo).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        .map(|_| StatusCode::CREATED)
}
// curl to test create todo
// curl -X POST -H "Content-Type: application/json" -d '{"text":"hello world"}' http://localhost:3002/todos