mod fixture;

use fixture::Fixture;
use serde::Deserialize;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::collections::HashMap;
use rand::Rng;
use tower_http::cors::{CorsLayer, Any};
use axum::{
    Json,
    routing::post,
    Router,
    http,
    http::header,
    extract::State,
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

    // Create database
    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://quasarcup.db")
        .await
        .expect("Failed to connect to database");

    // Build the application with routes
    let app = Router::new()
        // Routes with get() or post() methods, each will call a handler
        .route("/fixture", post(make_fixture))
        .with_state(db.clone())
        .layer(cors);      

    // Tcp Listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Run app
    axum::serve(listener, app).await.unwrap();
}

async fn make_fixture(
    State(db): State<SqlitePool>,
    Json(payload): Json<FixtureMakerInput>
) -> axum::Json<Fixture> {
    let fixture = Fixture::create_fixture(payload.team_number);
    let code = generate_code(&db).await;
    let name = payload.tournament;

    // TODO: Add match result for error handling
    let _result = save_fixture_to_database(&db, &fixture, code, name).await;

    axum::Json(
        fixture
    )
}

async fn save_fixture_to_database(
    db: &SqlitePool, 
    fixture: &Fixture, 
    code: String,
    name: String
) -> Result<i64, sqlx::Error> {    
    // First create a tournament, returning its id for later use.
    let tournament_id = sqlx::query!(
        "INSERT INTO tournaments (name, code) VALUES (?, ?) RETURNING id", name, code
    ).fetch_one(db).await?.id;

    // Create teams, hashing name and id for later use
    let mut team_id_map = HashMap::new();
    for team in fixture.teams.iter() {
        if team.name == "FREE" { continue; } // Don't insert "FREE" team

        let t = sqlx::query!("INSERT INTO teams (tournament_id, name) VALUES (?, ?) RETURNING id", 
            tournament_id, team.name
        )
        .fetch_one(db)
        .await?;

        team_id_map.insert(team.name.clone(), t.id); // Insert team to HashMap
    }

    // Finally, insert every game from every date into database
    for date in fixture.dates.iter() {
        for game in date.games.iter() {
            // Dont create games for FREE dates
            if game.home_team.name == "FREE" || game.away_team.name == "FREE" {
                continue;
            }

            // Ensure that team exists (it should always)
            if let (Some(&home_team_id), Some(&away_team_id)) = (
                team_id_map.get(&game.home_team.name),
                team_id_map.get(&game.away_team.name)
            ) {
                // Insert game
                sqlx::query!(
                    "INSERT INTO games (tournament_id, date_idx, game_idx, home_team_id, away_team_id) 
                    values (?, ?, ?, ?, ?)", 
                    tournament_id, game.date_idx, game.game_idx, home_team_id, away_team_id
                ).execute(db).await?;
            }
        }
    }
    // Return Ok if all goes well, other wise the '?' operator will return a sqlx::Error
    Ok(tournament_id)
}

async fn generate_code(db: &SqlitePool) -> String {
    loop {
        let code = format!("{:04}", rand::rng().random_range(0..=9999));
        let exists: Option<i64> = sqlx::query_scalar( // Check if random generated code exists in database
            "SELECT 1 FROM tournaments WHERE code = ?"
        )
        .bind(&code)
        .fetch_optional(db)
        .await
        .unwrap();

        if exists.is_none() { // Return if it is unique, other wise loop and try again
            return code;
        }    
    }
}
