use std::net::SocketAddr;
use std::sync::Arc;

use aide::{
    axum::ApiRouter,

};
use aide::openapi::OpenApi;
use axum::{Extension, http::StatusCode};
use axum::response::IntoResponse;
use log::LevelFilter;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use state::AppState;

use crate::docs::docs_routes;
use crate::routers::main_router;

pub mod errors;
pub mod state;
pub mod routers;

pub mod docs;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    aide::gen::on_error(|error| {
        println!("{error}");
    });

    env_logger::builder().filter_level(LevelFilter::Info).init();


    aide::gen::extract_schemas(true);


    let state = AppState::new();


    let mut api = OpenApi::default();

    let app = ApiRouter::new()
        .nest_api_service("/", main_router(state.clone()))
        .nest_api_service("/docs", docs_routes(state.clone()))
        .finish_api_with(&mut api, docs::api_docs)
        .layer(Extension(Arc::new(api)))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(state);

    let app = app.fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    ).await.unwrap();
}



async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}