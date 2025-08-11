use serde::Serialize;
use sqlx::SqlitePool;
use std::{vec, collections::HashMap};

#[derive(Serialize, Debug, Clone)] // For handling JSON
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
#[derive(Serialize, Debug, Clone)] // For handling JSON
pub struct Game {
    // Date and game
    pub game_idx: i32,
    pub date_idx: i32,
    // Teams
    pub home_team: Team,
    pub away_team: Team,
    // Score
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
}

#[derive(Serialize, Debug)] // For handling JSON
pub struct Date {
    pub games: Vec<Game>,
    pub date_idx: usize,
}

impl Date {
    fn new(games: Vec<Game>, date_idx: usize) -> Date {
        Date { games, date_idx }
    }
}

#[derive(Serialize, Debug)]
pub struct Tournament {
    pub name: String,
    pub teams: Vec<Team>,
    pub matches: Vec<Date>
}

impl Tournament {
    pub fn new(name: String, team_number: usize) -> Tournament {
        let teams = create_teams(team_number);
        let matches = create_matches(teams.clone());
        Tournament { name, teams, matches }
    }

    // TODO:
    // Ok so I get a response alr?!
    // No unwrap error or expect, nothing! GREAT
    // but the returned tournament is not exaclty what it should be 
    // more specifically in matches, its weird
    // still have not figured out the error, or where it deserializes wrong
    // but there's def something wrong with the order of the dates 
    // ik my fixtures they aint like that!
    pub async fn deserialize_from_db(code: String, db: &SqlitePool) -> Option<Tournament> {
        let name;
        let tournament_id;
        {
            let row = match sqlx::query!(
                "SELECT id, name FROM tournaments WHERE code = ?",
                code
            ).fetch_optional(db).await.expect("wrong code") {
                Some(x) => x,
                None => {
                    return None;
                }
            };

            name = row.name;
            tournament_id = row.id;
        }
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

        let mut matches = vec![];
        {
            let rows = sqlx::query!(
                "SELECT date_idx, game_idx, home_team_id, away_team_id, home_score, away_score FROM games WHERE tournament_id = ?",
                tournament_id
            ).fetch_all(db).await.expect("Failed to fetch matches");

            let mut games_by_date: HashMap<usize, Vec<Game>> = HashMap::new();

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
                games_by_date.entry(row.game_idx as usize).or_default().push(game);
            }
            
            for date_idx in games_by_date.keys() {
                matches.push(Date::new(
                    games_by_date[date_idx].clone(),
                    *date_idx
                ));
            }
        }

        println!("{:?}", matches);

        // get data from data base using code
        // send fixture if it exists back to the front end 
        // enought to render
        // send None if code is not valid

        Some(Tournament { name, teams, matches })
    }
}

fn create_teams(n: usize) -> Vec<Team> {
    let mut teams = vec![];
    for i in 1..=n {
        teams.push(Team::new(format!("team{i}")));
    }

    teams
}

pub fn create_matches(mut teams: Vec<Team>) -> Vec<Date> {
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

    create_fixture(date_num, teams.clone(), len)
}

fn create_fixture(date_num: usize, mut teams: Vec<Team>, len: usize) -> Vec<Date> {
    // Use Round-Robin Circle algorithm for creating the fixture
    let mut dates = vec![];

    for date_idx in 0..date_num {
        let date_games = create_date_games(teams.clone(), date_idx, len);
        dates.push(Date::new(date_games, date_idx));

        // Then cicle through
        let last = teams.pop().unwrap(); // Move the last team to the second position
        teams.insert(1, last); // Pos 0 is fixed
    }

    dates
}

fn create_date_games(teams: Vec<Team>, date_idx: usize, len: usize) -> Vec<Game> {
    // For each date, arrange games by pairing first and last
    // circle algorithm
    let mut date_games = vec![];
    for game_idx in 0..len / 2 {
        date_games.push({
            // Switch team1 to play both as home and away
            if is_pair(date_idx) {
                Game::new(
                    teams[game_idx].clone(),
                    teams[len - game_idx - 1].clone(),
                    game_idx.try_into().unwrap(),
                    date_idx.try_into().unwrap(),
                )
            } else {
                Game::new(
                    teams[len - game_idx - 1].clone(),
                    teams[game_idx].clone(),
                    game_idx.try_into().unwrap(),
                    date_idx.try_into().unwrap(),
                )
            }
        });
    }
    date_games
}

fn is_pair(number: usize) -> bool {
    if number % 2 == 0 {
        return true;
    }
    false
}
