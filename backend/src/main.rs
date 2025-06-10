mod fixture;

use fixture::Fixture;
use tower_http::cors::{CorsLayer, Any};
use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    // add a Cross-Origin Resource Sharing (cors) middleware
    let cors = CorsLayer::new().allow_origin(Any);

    // build our application with routes
    let app = Router::new()
        .route("/", get(handler)) // I dont really want to GET this route temp
        .route("/fixture", post(make_fixture))
        .layer(cors);      

    // *TODO:
    // Implement POST request for "/fixture" DONE
    // Create the actual fixute DOING
    // Send response back to the frontend

    // run app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello backend!"
}

async fn make_fixture() -> &'static str{
    let _teams = Fixture::create_fixture(6).teams;
    "Teams"
}
