use std::vec;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)] // For handling JSON
pub struct Team {
    pub name: String,
}

impl Team {
    pub fn new(name: String) -> Team {
        Team { name }
    }
}

#[derive(Serialize, Debug)] // For handling JSON
pub struct Game {
    pub game_idx: i32,
    pub date_idx: i32,
    pub home_team: Team,
    pub away_team: Team,
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
            away: 0
        }
    }
    // TODO:
    // score function
}

#[derive(Serialize, Debug)] // For handling JSON
pub struct Date {
    pub games: Vec<Game>,
}

impl Date {
    fn new(games: Vec<Game>) -> Date {
        Date { games }
    }
}

#[derive(Serialize, Debug)] // For handling JSON
pub struct Fixture {
    pub teams: Vec<Team>,
    pub dates: Vec<Date>,
}

impl Fixture {
    pub fn create_fixture(n: usize) -> Fixture {
        let mut games = vec![];
        let mut teams = vec![];
        
        // Create vector of n teams
        for i in 1..=n {
            teams.push(Team::new(format!("team{i}")));
        }

        // Create a vector of all posible matches 
        for i in 0..n {
            for j in i+1..n {
                games.push(Game::new(
                    teams.get(i).unwrap().clone(),
                    teams.get(j).unwrap().clone(),
                    0,
                    0
                ));
            }
        }

        // Free date if not pair amount of teams
        let mut len = teams.len();
        if len % 2 != 0 {
            teams.push(Team::new(String::from("FREE")));
            len = teams.len();
        }

        let date_num = len - 1;
        let mut dates = vec![];

        // Circle algorithm tingy
        for date_idx in 0..date_num {
            // For each date, arrange games by pairing first and last
            let mut date_games = vec![];
            for game_idx in 0..len / 2 {
                date_games.push( {
                    if date_idx % 2 == 0 { // Just avoid team1 to play aways as home
                        Game::new(
                            teams[game_idx].clone(),
                            teams[len - game_idx - 1].clone(),
                            game_idx.try_into().unwrap(),
                            date_idx.try_into().unwrap()
                        )
                    } else {
                        Game::new(
                            teams[len - game_idx - 1].clone(),
                            teams[game_idx].clone(),
                            game_idx.try_into().unwrap(),
                            date_idx.try_into().unwrap()
                        )
                    }
                } );
            }
            dates.push(Date::new(date_games));

            // Then cicle through
            let last = teams.pop().unwrap(); // Move the last team to the second position
            teams.insert(1, last); // Pos 0 is fixed
        }

        Fixture {
            teams,
            dates,
        }
    }
}
