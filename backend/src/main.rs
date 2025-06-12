mod fixture;

use fixture::Fixture;
use serde::{Serialize, Deserialize};
use tower_http::cors::{CorsLayer, Any};
use axum::{
    Json,
    routing::{get, post},
    Router,
    http::header,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)] // For handling as JSON 
pub struct FixtureMakerInput {
    pub tournament: String,
    pub team_number: i32,
}

#[tokio::main]
async fn main() {
    // add a Cross-Origin Resource Sharing (cors) middleware
    let cors = CorsLayer::new()
        .allow_origin(Any) // Anyone can access the app
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::OPTIONS]) 
        .allow_headers([header::CONTENT_TYPE]);

    // build our application with routes
    let app = Router::new()
        // Routes with get() or post() methods, each will call a handler
        .route("/", get(handler)) // I dont really want to GET this route temp
        .route("/fixture", post(make_fixture))
        .layer(cors);      

    // *TODO:
    // Implement POST request for "/fixture" DONE
    // Create the actual fixute DOING
    // Send response back to the frontend

    // run app axum::serve 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello backend!"
}

async fn make_fixture(Json(payload): Json<FixtureMakerInput>) -> String {
    let fix = Fixture::create_fixture(payload.team_number);
    format!("Fixture {:?}", fix)
}
