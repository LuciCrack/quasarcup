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
    pub home: u16,
    pub away: u16,
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

#[derive(Serialize, Deserialize, Debug)] // For handling JSON
pub struct Fixture {
    pub teams: Vec<Team>,
    pub dates: Vec<Date>,
}

impl Fixture {
    pub fn create_fixture(team_number: u16) -> Fixture {
        let mut teams = vec![];
        for i in 0..team_number {
            teams.push(Team::new(format!("team{i}")));
        }
        println!("{:?}", teams);
        Fixture {
            teams,
            dates: vec![],
        }
    }
}
