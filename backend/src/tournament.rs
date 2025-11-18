use serde::Serialize;
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
            "SELECT date_idx, game_idx, home_team_id, away_team_id, home_score, away_score 
            FROM games WHERE tournament_id = ?",
            tournament_id
        ).fetch_all(db).await.expect("Failed to fetch matches");

        //                              date, games
        let mut matches: BTreeMap<usize, Vec<Game>> = BTreeMap::new();

        for row in rows.iter() {
            // Get playing teams from id's 
            let home_name = sqlx::query!(
                "SELECT name FROM teams WHERE id = ?",
                row.home_team_id
            ).fetch_one(db).await.expect("Failed to fetch home team").name;
            let home_team = Team::new(home_name);
            let away_name = sqlx::query!(
                "SELECT name FROM teams WHERE id = ?",
                row.away_team_id
            ).fetch_one(db).await.expect("Failed to fetch home team").name;
            let away_team = Team::new(away_name);

            // Create new game 
            let game = Game::with_score(
                home_team, 
                away_team, 
                row.game_idx as i32, 
                row.date_idx as i32, 
                row.home_score as i32, 
                row.away_score as i32
            );
            matches.entry(row.date_idx as usize).or_default().push(game);
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
        println!("[DEBUG] Searching for code:'{}'", code);

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

        let (name, tournament_id) = Tournament::get_name_and_id(code, db).await.expect("Wrong code! or smth");

        let mut teams = vec![];
        {
            let rows = sqlx::query!(
                "SELECT name FROM teams WHERE tournament_id = ?",
                tournament_id
            ).fetch_all(db).await.expect("Failed to fetch teams");

            for row in rows.iter() {
                teams.push(
                    Team::new(row.name.clone())
                );
            }
        }

        let matches = Game::get_matches(tournament_id, db).await;
            
        Some(Tournament { name, teams, matches })
    }

    pub async fn create_to_database(
        db: &SqlitePool,
        tournament: &Tournament,
        code: &String,
    ) -> Result<i64, sqlx::Error> {
        // First create a tournament, returning its id for later use.
        let tournament_id = sqlx::query!(
            "INSERT INTO tournaments (name, code) VALUES (?, ?) RETURNING id",
            tournament.name,
            code
        )
        .fetch_one(db)
        .await?
        .id;

        // Create teams, hashing name and id for later use
        let mut team_id_map = HashMap::new();
        for team in tournament.teams.iter() {
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
        for games in tournament.matches.values() {
            for game in games.iter() {
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
