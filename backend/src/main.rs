mod fixture;

use fixture::Fixture;
use serde::Deserialize;
use tower_http::cors::{CorsLayer, Any};
use axum::{
    Json,
    routing::post,
    Router,
    http,
    http::header,
};

#[derive(Deserialize)] // For handling as JSON 
pub struct FixtureMakerInput {
    pub tournament: String,
    pub team_number: usize,
}

#[tokio::main]
async fn main() {
    // add a Cross-Origin Resource Sharing (cors) middleware
    let cors = CorsLayer::new()
        .allow_origin(Any) // Anyone can access the app
        .allow_methods([http::Method::GET, http::Method::POST, http::Method::OPTIONS]) 
        .allow_headers([header::CONTENT_TYPE]);

    // Build the application with routes
    let app = Router::new()
        // Routes with get() or post() methods, each will call a handler
        .route("/fixture", post(make_fixture))
        .layer(cors);      

    // run app
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn make_fixture(Json(payload): Json<FixtureMakerInput>) -> axum::Json<Fixture> {
    axum::Json(
        Fixture::create_fixture(payload.team_number)
    )
}
