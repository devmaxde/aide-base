use aide::axum::ApiRouter;

use crate::state::AppState;

pub mod todos;

pub fn main_router(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .nest_api_service("/todo", todos::routes::todo_routes(state.clone()))
        .with_state(state)
}
