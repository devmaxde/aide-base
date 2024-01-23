use std::sync::Arc;

use aide::{
    axum::routing::get_with,
    openapi::OpenApi,
    redoc::Redoc,
};
use aide::axum::{ApiRouter, IntoApiResponse};
use aide::openapi::Tag;
use aide::transform::TransformOpenApi;
use axum::Extension;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum_extra::response::Html;
use hyper::StatusCode;
use uuid::Uuid;

use crate::docs::extractors::Json;
use crate::errors::AppError;
use crate::state::AppState;

pub mod extractors;

pub fn docs_routes(state: AppState) -> ApiRouter {
    aide::gen::infer_responses(true);

    let router = ApiRouter::new()
        .route("/openapi.json", get_with(serve_docs, |p| p.tag("docs")))
        .api_route_with(
            "/redoc",
            get_with(
                Redoc::new("/docs/openapi.json")
                    .with_title("Aide Axum")
                    .axum_handler(),
                |op| op.description("This documentation page.").tag("docs"),
            ),
            |p| { p.security_requirement("ApiKey").tag("docs") },
        )
        .api_route("/swagger/", get_with(index, |p| p.tag("docs")))
        .api_route("/", get_with(index, |p| p.tag("docs")))

        .api_route("/static/*path", get_with(static_assets, |p| p.tag("docs")))
        .api_route("/swagger/static/*path", get_with(static_assets, |p| p.tag("docs")))
        .with_state(state);
    aide::gen::infer_responses(false);
    router
}

async fn index() -> impl IntoApiResponse {
    let html = include_str!("index.html");
    Html(html).into_response()
}

async fn static_assets(Path(path): Path<String>) -> impl IntoApiResponse {
    println!("path: {}", path);
    if path == "swagger-ui.css" {
        let css = include_str!("static/swagger-ui.css");
        return (StatusCode::OK, [("Content-Type", "text/css")], css.into_response());
    } else if path == "swagger-ui-bundle.js" {
        let js = include_str!("static/swagger-ui-bundle.js");
        return (StatusCode::OK, [("Content-Type", "text/javascript")], js.into_response());
    } else {
        return (StatusCode::NOT_FOUND, [("Content-Type", "text/plain")], "Not Found".into_response());
    }
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}


pub(crate) fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Aide axum Open API")
        .summary("An example Todo application")
        .description(include_str!("static/README.md"))
        .tag(Tag {
            name: "todo".into(),
            description: Some("Todo Management".into()),
            ..Default::default()
        })
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "X-Auth-Key".into(),
                description: Some("A key that is ignored.".into()),
                extensions: Default::default(),
            },
        )
        .default_response_with::<Json<AppError>, _>(|res| {
            res.example(AppError {
                error: "some error happened".to_string(),
                error_details: None,
                error_id: Uuid::nil(),
                // This is not visible.
                status: StatusCode::IM_A_TEAPOT,
            })
        })
}