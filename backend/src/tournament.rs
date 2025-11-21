use serde::Serialize;
use log::{info, debug};
use sqlx::SqlitePool;
use std::{collections::{HashMap, BTreeMap}, vec};

use super::UpdateMatch;

pub enum TeamRole {
    Home,
    Away,
}

#[derive(Serialize, Debug, Clone)]
pub struct Team {
    pub name: String,
}

impl Team {
    pub fn new(name: String) -> Team {
        Team { name }
    }
}

// TODO:
// Free Team
#[derive(Serialize, Debug, Clone)]
pub struct Game {
    // Date and game
    pub game_idx: i32,
    pub date_idx: i32,
    // Teams
    pub home_team: Team,
    pub away_team: Team,
    // Scores
    pub home: i32,
    pub away: i32,
}

impl Game {
    fn new(home_team: Team, away_team: Team, game_idx: i32, date_idx: i32) -> Game {
        Game {
            game_idx,
            date_idx,
            home_team,
            away_team,
            home: 0,
            away: 0,
        }
    }
    fn with_score(home_team: Team, away_team: Team, game_idx: i32, date_idx: i32, home: i32, away: i32) -> Game {
        Game {
            game_idx,
            date_idx,
            home_team,
            away_team,
            home,
            away,
        }
    }
    // TODO:
    // score function
    fn score(&mut self, team_role: TeamRole) {
        match team_role {
            TeamRole::Home => self.home += 1,
            TeamRole::Away => self.away += 1,
        }
    }

    fn update_match(&mut self, home: i32, away: i32) {
        self.home = home;
        self.away = away;
    }

    async fn get_matches(tournament_id: i64, db: &SqlitePool) -> BTreeMap<usize, Vec<Game>> {
        let rows = sqlx::query!(
            "SELECT 
                g.date_idx, 
                g.game_idx, 
                g.home_score, 
                g.away_score,
                ht.name as home_team_name,
                at.name as away_team_name
            FROM games g
            JOIN teams ht ON g.home_team_id = ht.id
            JOIN teams at ON g.away_team_id = at.id
            WHERE g.tournament_id = ?
            ORDER BY g.date_idx, g.game_idx",
            tournament_id
        )
        .fetch_all(db)
        .await
        .expect("Failed to fetch matches");

        let mut matches: BTreeMap<usize, Vec<Game>> = BTreeMap::new();

        for row in rows {
            let game = Game::with_score(
                Team::new(row.home_team_name),
                Team::new(row.away_team_name),
                row.game_idx as i32,
                row.date_idx as i32,
                row.home_score as i32,
                row.away_score as i32,
            );
            
            matches
                .entry(row.date_idx as usize)
                .or_default()
                .push(game);
        }

        matches
    }
}

#[derive(Serialize, Debug)]
pub struct Tournament {
    pub name: String,
    pub teams: Vec<Team>,
    pub matches: BTreeMap<usize, Vec<Game>>
}

impl Tournament {
    pub fn new(name: String, team_number: usize) -> Tournament {
        let teams = create_teams(team_number);
        let matches = create_matches(teams.clone());
        Tournament { name, teams, matches }
    }

    pub async fn exists(code: String, db: &SqlitePool) -> bool {
        let code = code.trim().trim_matches('"').to_string();
        sqlx::query!(
            "SELECT id FROM tournaments WHERE code = ?",
            code
        ).fetch_optional(db).await.expect("Failed to fetch id").is_some()
    }

    pub async fn get_id(code: String, db: &SqlitePool) -> Option<i64> {
        let code = code.trim().trim_matches('"').to_string();

        match sqlx::query!(
            "SELECT id FROM tournaments WHERE code = ?",
            code
        ).fetch_optional(db).await.expect("no code") {
            Some(x) => x.id,
            _ => None,
        }
    }

    pub async fn get_name_and_id(code: String, db: &SqlitePool) -> Option<(String, i64)> {
        let code = code.trim().trim_matches('"').to_string();
        debug!("Searching for code:'{}'", code);

        let name;
        let tournament_id;
        {
            let row = match sqlx::query!(
                "SELECT id, name FROM tournaments WHERE code = ?",
                code
            ).fetch_optional(db).await.expect("wrong database or smth") {
                Some(x) => x,
                None => return None,
            };
            name = row.name;
            tournament_id = row.id.unwrap();
        }
        Some((name, tournament_id))
    }

    pub async fn deserialize_from_db(code: String, db: &SqlitePool) -> Option<Tournament> {
        let code = code.trim().trim_matches('"').to_string();

        // Single query for all tournament data
        let tournament_data = sqlx::query!(
            "SELECT 
                t.id as tournament_id,
                t.name as tournament_name,
                team.name as team_name
            FROM tournaments t
            LEFT JOIN teams team ON t.id = team.tournament_id
            WHERE t.code = ?",
            code
        )
        .fetch_all(db)
        .await
        .ok()?;

        if tournament_data.is_empty() {
            return None;
        }

        // Extract tournament info from first row (name and id)
        let first_row = &tournament_data[0];
        let name = first_row.tournament_name.clone();
        let tournament_id = first_row.tournament_id?;

        // Build teams list
        let mut teams = Vec::new();
        for row in &tournament_data {
            teams.push(Team::new(row.team_name.clone()));
        }

        // Get matches
        let matches = Game::get_matches(tournament_id, db).await;

        Some(Tournament { name, teams, matches })
    }

    pub async fn create_to_database(
        db: &SqlitePool,
        tournament: &Tournament,
        code: &String,
    ) -> Result<i64, sqlx::Error> {
        // Use transaction for atomic operations
        let mut transaction = db.begin().await?;

        // Create tournament
        let tournament_row = sqlx::query!(
            "INSERT INTO tournaments (name, code) VALUES (?, ?) RETURNING id",
            tournament.name,
            code
        )
        .fetch_one(&mut *transaction)
        .await?;

        let tournament_id = tournament_row.id;

        // Batch insert teams
        let team_names: Vec<&String> = tournament.teams
            .iter()
            .filter(|team| team.name != "FREE")
            .map(|team| &team.name)
            .collect();

        if !team_names.is_empty() {
            let mut query_builder = sqlx::QueryBuilder::new(
                "INSERT INTO teams (tournament_id, name) "
            );
            
            query_builder.push_values(team_names, |mut b, team_name| {
                b.push_bind(tournament_id)
                 .push_bind(team_name);
            });

            query_builder.build().execute(&mut *transaction).await?;
        }

        // Get team IDs
        let team_rows = sqlx::query!(
            "SELECT name, id FROM teams WHERE tournament_id = ?",
            tournament_id
        )
        .fetch_all(&mut *transaction)
        .await?;

        let team_id_map: HashMap<String, i64> = team_rows
            .into_iter()
            .map(|row| (row.name, row.id))
            .collect();

        // Batch insert games
        let mut games_to_insert = Vec::new();
        for games in tournament.matches.values() {
            for game in games.iter() {
                if game.home_team.name != "FREE" && game.away_team.name != "FREE" {
                    if let (Some(&home_id), Some(&away_id)) = (
                        team_id_map.get(&game.home_team.name),
                        team_id_map.get(&game.away_team.name),
                    ) {
                        games_to_insert.push((
                            tournament_id,
                            game.date_idx,
                            game.game_idx,
                            home_id,
                            away_id,
                        ));
                    }
                }
            }
        }

        if !games_to_insert.is_empty() {
            let mut query_builder = sqlx::QueryBuilder::new(
                "INSERT INTO games (tournament_id, date_idx, game_idx, home_team_id, away_team_id) "
            );
            
            query_builder.push_values(games_to_insert, |mut b, (tournament_id, date_idx, game_idx, home_id, away_id)| {
                b.push_bind(tournament_id)
                 .push_bind(date_idx)
                 .push_bind(game_idx)
                 .push_bind(home_id)
                 .push_bind(away_id);
            });

            query_builder.build().execute(&mut *transaction).await?;
        }

        // Commit all changes
        transaction.commit().await?;

        // Ok transactions are amazing wtf
        info!("Successfully created and stored tournament {:?}", tournament.name);
        Ok(tournament_id)
    }

    pub async fn update_match_to_db(update: UpdateMatch, db: &SqlitePool) -> Result<bool, sqlx::Error> {
        if let Some(id) = Tournament::get_id(update.code, &db).await {
            // Query and update match
            let res = sqlx::query!(
                "UPDATE games
                 SET home_score = ?, away_score = ?
                 WHERE tournament_id = ? AND date_idx = ? AND game_idx = ?",
                update.home, update.away, id, update.date_idx, update.game_idx
            )
            .execute(db)
            .await?;

            // rows_affected > 0 means we actually updated something
            Ok(res.rows_affected() > 0)
        } else {
            // tournament not found
            Ok(false)
        }
    }
}


fn create_teams(n: usize) -> Vec<Team> {
    let mut teams = vec![];
    for i in 1..=n {
        teams.push(
            Team::new(format!("team{i}"))
        );
    }

    teams
}

pub fn create_matches(mut teams: Vec<Team>) -> BTreeMap<usize, Vec<Game>> {
    // Create Vector of Dates (fixture)
    // One way, Free For All using Round-Robin algorithm
    // For more information on Round-Robin for sports visit 
    // https://medium.com/coinmonks/sports-scheduling-simplified-the-power-of-the-rotation-algorithm-in-round-robin-tournament-eedfbd3fee8e

    // Free date if not pair amount of teams
    let mut len = teams.len();
    if !is_pair(len) {
        teams.push(Team::new(String::from("FREE")));
        len = teams.len();
    }

    // How many dates there will be
    let date_num = len - 1;

    let mut matches: BTreeMap<usize, Vec<Game>> = BTreeMap::new();

    for date_idx in 0..date_num {
        for game_idx in 0..len/2 {
            // Compiler will tell you "unneeded late initialization" or something..
            // Ignore it, it is needed :D
            let game;
            if is_pair(date_idx) {
                game = Game::new(
                    teams[game_idx].clone(),
                    teams[len - game_idx - 1].clone(),
                    game_idx.try_into().unwrap(),
                    date_idx.try_into().unwrap(),
                )
            } else {
                game = Game::new(
                    teams[len - game_idx - 1].clone(),
                    teams[game_idx].clone(),
                    game_idx.try_into().unwrap(),
                    date_idx.try_into().unwrap(),
                )
            }
            matches.entry(date_idx).or_default().push(game);
        }

        // Then cicle through
        let last = teams.pop().unwrap(); // Move the last team to the second position
        teams.insert(1, last); // Pos 0 is fixed
    }

    matches
}

fn is_pair(number: usize) -> bool {
    if number % 2 == 0 {
        return true;
    }
    false
}
