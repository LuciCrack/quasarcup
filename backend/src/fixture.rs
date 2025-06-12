use std::vec;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)] // For handling JSON
pub struct Team {
    pub name: String,
}

impl Team {
    pub fn new(name: String) -> Team {
        Team {
            name,
        }
    }
}

// Might change
#[derive(Serialize, Deserialize, Debug)] // For handling JSON
pub struct Game {
    pub home_team: String,
    pub away_team: String,
    pub home: i32,
    pub away: i32,
}

impl Game {
    fn new(home_team: &str, away_team: &str) -> Game {
        Game {
            home_team: home_team.to_string(),
            away_team: away_team.to_string(),
            home: 0,
            away: 0
        }
    }
    // TODO:
    // score function
}

#[derive(Serialize, Deserialize, Debug)] // For handling JSON
pub struct Date {
    pub games: Vec<Game>,
    pub free_team: Option<Team>,
}

impl Date {
    fn new(games: Vec<Game>, free_team: Option<Team>) -> Date {
        Date {
            games,
            free_team,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)] // For handling JSON
pub struct Fixture {
    pub teams: Vec<Team>,
    pub dates: Vec<Date>,
}

impl Fixture {
    pub fn create_fixture(team_number: i32) -> Fixture {
        let mut rng = rand::rng();
        let mut teams = vec![];

        let mut indexes: Vec<i32> = (1..=team_number).collect();
        for i in indexes.iter() {
            teams.push(Team::new(format!("team{i}")));
        }

        // TODO:
        // Create games for each dates and fixture from all dates
        //
        // Create first date from shuffled indexes
        // Then create all other dates that do not repeat a game (back or forth)

        Fixture {
            teams,
            dates: vec![],
        }
    }
}
