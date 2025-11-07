mod tournament;

use axum::{
    Json, 
    Router, 
    extract::State, 
    http::{self, header},
    routing::post
};

use tournament::Tournament;
use rand::Rng;
use serde::Deserialize;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // add a Cross-Origin Resource Sharing (cors) middleware
    let cors = CorsLayer::new()
        .allow_origin(Any) // Anyone can access the app
        .allow_methods([http::Method::GET, http::Method::POST, http::Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE]);

    // Create database
    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://quasarcup.db")
        .await
        .expect("Failed to connect to database");

    // Build the application with routes

    let app = Router::new()
        // Routes with get() or post() methods, each will call a handler
        .route("/create_tournament", post(create_tournament))
        .route("/reset_database", post(nuke_database))
        .route("/exists_tournament", post(exists_tournament))
        .route("/get_tournament", post(get_tournament))
        .route("/update_match", post(update_match))
        .with_state(db.clone())
        .layer(cors)
        // Serve static files from the frontend/ directory
        .nest_service("/", ServeDir::new("frontend"))
        // Fallback to index.html for client-side routing
        .fallback_service(ServeDir::new("frontend").append_index_html_on_directories(true));

    // Tcp Listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    // Run app
    axum::serve(listener, app).await.unwrap();
}

async fn create_tournament(
    State(db): State<SqlitePool>,
    Json(input): Json<CreateTournamentInput>,
) -> String {
    let tournament = Tournament::new(input.tournament_name, input.team_number);
    let code = generate_code(&db).await;

    // TODO: Add match result for error handling
    let _result = Tournament::create_to_database(&db, &tournament, &code).await;

    code
}

async fn exists_tournament(
    State(db): State<SqlitePool>, 
    code: String
) -> axum::Json<bool> {
    Json (
        Tournament::exists(code, &db).await
    )
}

async fn get_tournament(
    State(db): State<SqlitePool>,
    code: String,
) -> axum::Json<Option<Tournament>> {
    let tournament = Tournament::deserialize_from_db(code, &db).await;

    Json(
        tournament
    )
}

async fn update_match(
    State(db): State<SqlitePool>,
    Json(input): Json<UpdateMatch>,
) -> axum::Json<bool> {
    let result =  Tournament::update_match_to_db(input, &db).await
        .expect("Failed to update database");

    println!("Result: {result}");
    Json (result)
}

async fn nuke_database(State(db): State<SqlitePool>, input: String) -> String {
    // Check password (really lame check) and reset database
    if input == "0123456789" {
        sqlx::query!("DELETE FROM games")
            .execute(&db)
            .await
            .expect("COULD NOT DELETE games TABLE");
        sqlx::query!("DELETE FROM teams")
            .execute(&db)
            .await
            .expect("COULD NOT DELETE teams TABLE");
        sqlx::query!("DELETE FROM tournaments")
            .execute(&db)
            .await
            .expect("COULD NOT DELETE tournaments TABLE");
    } else {
        return "WRONG PASSWORD".to_string();
    }

    "ALL GOOD".to_string()
}

async fn generate_code(db: &SqlitePool) -> String {
    loop {
        let code = format!("{:04}", rand::rng().random_range(0..=9999));
        let exists: Option<i64> = sqlx::query_scalar(
            // Check if random generated code exists in database
            "SELECT 1 FROM tournaments WHERE code = ?",
        )
        .bind(&code)
        .fetch_optional(db)
        .await
        .unwrap();

        if exists.is_none() {
            // return and stop loop if code is unique
            // other wise loop and try again
            return code;
        }
    }
}

#[derive(Deserialize)] // For handling as JSON 
pub struct CreateTournamentInput {
    pub tournament_name: String,
    pub team_number: usize,
}

#[derive(Deserialize)]
pub struct UpdateMatch {
    code: String,
    game_idx: i32,
    date_idx: i32,
    home: i32,
    away: i32,
}
