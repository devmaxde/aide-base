use aide::{
    axum::{
        ApiRouter,
        IntoApiResponse, routing::get_with,
    },
    transform::TransformOperation,
};
use axum::extract::State;
use schemars::JsonSchema;
use serde::Serialize;

use crate::{state::AppState};
use crate::docs::extractors::Json;

pub fn todo_routes(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list_todos, list_todos_docs),
        )
        .with_state(state)
}

#[derive(Serialize, JsonSchema)]
struct TodoList {
    todo_ids: Vec<i32>,
}

async fn list_todos(State(state): State<AppState>) -> impl IntoApiResponse {

    Json(TodoList { todo_ids: vec![] })
}

fn list_todos_docs(op: TransformOperation) -> TransformOperation {
    op.description("List all Todo items.")
        .response::<200, Json<TodoList>>()
}

