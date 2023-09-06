mod todo_index;
mod todo_create;
mod todo_update;
mod todo_delete;

use axum::http::StatusCode;
pub use todo_index::*;
pub use todo_create::*;
pub use todo_update::*;
pub use todo_delete::*;
fn internal_error(err: anyhow::Error) -> (StatusCode, String)

{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
