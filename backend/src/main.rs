mod fixture;

use axum::{
    Json, 
    Router, 
    extract::State, 
    http::{self, header},
    routing::{post, get}
};
use fixture::Fixture;
use rand::Rng;
use serde::Deserialize;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::collections::HashMap;
use tower_http::cors::{Any, CorsLayer};

// TODO: refractor structures
// Tournament: - Name
//             - Fixture or straight up Dates
//             - Teams

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
        .route("/get_tournament", get(get_tournament))
        .with_state(db.clone())
        .layer(cors);

    // Tcp Listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Run app
    axum::serve(listener, app).await.unwrap();
}

async fn create_tournament(
    State(db): State<SqlitePool>,
    Json(input): Json<CreateTournamentInput>,
) -> axum::Json<String> {
    let fixture = Fixture::create_fixture(input.team_number);
    let code = generate_code(&db).await;
    let name = input.tournament_name;

    // TODO: Add match result for error handling
    let _result = save_fixture_to_database(&db, &fixture, &code, name).await;

    axum::Json(code)
}

async fn get_tournament(
    State(_db): State<SqlitePool>,
    _code: String,
) -> axum::Json<Option<Fixture>> {
    // TODO:
    // get data from data base using code
    // send fixture if it exists back to the front end 
    // enought to render
    // maybe not exactly Json<Fixture>
    // send None if code is not valid
    let fixture = None;
    Json(
        fixture
    )
}

async fn save_fixture_to_database(
    db: &SqlitePool,
    fixture: &Fixture,
    code: &String,
    name: String,
) -> Result<i64, sqlx::Error> {
    // First create a tournament, returning its id for later use.
    let tournament_id = sqlx::query!(
        "INSERT INTO tournaments (name, code) VALUES (?, ?) RETURNING id",
        name,
        code
    )
    .fetch_one(db)
    .await?
    .id;

    // Create teams, hashing name and id for later use
    let mut team_id_map = HashMap::new();
    for team in fixture.teams.iter() {
        if team.name == "FREE" {
            continue;
        } // Don't insert "FREE" team

        let t = sqlx::query!(
            "INSERT INTO teams (tournament_id, name) VALUES (?, ?) RETURNING id",
            tournament_id,
            team.name
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
                team_id_map.get(&game.away_team.name),
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
            // Return if it is unique, other wise loop and try again
            return code;
        }
    }
}

#[derive(Deserialize)] // For handling as JSON 
pub struct CreateTournamentInput {
    pub tournament_name: String,
    pub team_number: usize,
}
