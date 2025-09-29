use web_sys::{console, window};
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

use crate::pages::layout::Layout;

pub mod pages;

// TODO:
// Make page pretty

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/tournament")]
    TournamentCreate,
    #[at("/tournament/:code")]
    TournamentView { code: String },
    #[at("/dev")]
    Dev,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/search")]
    Search,
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html { 
    match route {
        Route::Home => html! { <Home /> },
        Route::TournamentCreate => html! { <TournamentCreate /> },
        Route::TournamentView { code } => html! { <TournamentView code={ code } /> },
        Route::Dev => html! { <Dev /> },
        Route::NotFound => html! { <div>{ "404 Not Found" }</div> },
        Route::Search => html! { <Search /> },
    }
}

// TODO:
// move function components for each route
// to a different module or something

#[function_component(Home)]
fn home() -> Html {
    html! {
        <Layout title="Home">
            <div>
                <h2> { "Welcome! Choose create or open tournament." } </h2>
            </div>
        </Layout>
    }
}

#[function_component(TournamentCreate)]
fn tournament_create() -> Html {
    // Variables to store and update local component state
    // holding the current value of the input field
    let tournament = use_state(|| "".to_string());
    let team_number: UseStateHandle<usize> = use_state(|| 0); 

    let navigator = use_navigator().unwrap();

    // Called when form is submited via button type="submit"
    let onsubmit = {
        // Cloning state handlers to use inside closure
        let tournament = tournament.clone();
        let team_number = team_number.clone();
        
        Callback::from(move |e: yew::SubmitEvent|{
            e.prevent_default(); // Prevent page reloading and Networks errors when posting

            // Collect input values to FixtureMakerInput struct
            let input = FixtureMakerInput { 
                tournament_name: (*tournament).clone(),
                team_number: (*team_number),
            };

            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let code = Request::post("http://localhost:2000/create_tournament")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&input).unwrap())
                    .unwrap().send().await
                    .expect("Failed to send post request")
                    .text().await.unwrap();
                
                navigator.push(&Route::TournamentView { code });
            });
        })


    };

    html! {
        <Layout title="Create Tournament">
            <div>
                <form {onsubmit}>
                    // html elements for user input
                    <input type="text" placeholder="Tournament Name"
                        // set value of the input box to the one stored in Yet state variable,
                        // derefencing UseStateHandle to get a String and cloning for ownership
                        value={(*tournament).clone()}
                        // event handler, closure takes ownership of tournament
                        oninput={Callback::from(move |e: InputEvent| {
                            // e is an event object from browser and bla bla bla (did not understand)
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            // .value() reads the input box and .set() updates the Yew state variable,
                            // triggering a re-render if it changes
                            tournament.set(input.value());
                        })}
                    />
                    <input type="number" min=2 max=999 placeholder="Number of Teams"
                        // same as the tournament, but intended for an i32 
                        value={(*team_number).clone().to_string()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            team_number.set(input.value().parse().unwrap_or(0));
                        })}
                    />
                    <button type="submit">{ "Create Tournament" }</button>
                </form>
            </div>        
        </Layout>
    }}

#[derive(Properties, PartialEq, Debug, Clone, Eq)]
struct TournamentViewProps {
    pub code: String,
}

#[function_component(Search)]
fn search() -> Html {
    let code: UseStateHandle<String> = use_state(|| "".to_string());
    let exists: UseStateHandle<Option<bool>> = use_state(|| None);

    let navigator = use_navigator().unwrap();

    let onsubmit = {
        let code = code.clone();
        let exists = exists.clone();

        Callback::from(move |e: yew::SubmitEvent|{
            e.prevent_default();

            let exists = exists.clone();
            let code = code.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::post("http://localhost:2000/exists_tournament")
                    .header("Content-Type", "application/json")
                    .body(&*code)
                    .unwrap().send().await
                    .expect("Failed to send post request");

                exists.set(resp.json().await.unwrap());
            });
        })
    };

    {
        let exists = exists.clone();
        let code = code.clone();

        match *exists {
            None => (),
            Some(true) => {
                navigator.push(&Route::TournamentView { code: code.to_string() });
                return html! { "Re-Routing" }
            },
            Some(false) => return html! { "Tournament not found, please try again" },
        }
    }

    html! {
        <Layout title="Search">
            <div>
                <p2> { "Search for a Tournament" } </p2>
                <form {onsubmit}>
                    <input type="text" placeholder="Tournament Code"
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            code.set(input.value());
                        })}
                    />
                    <button type="submit"> { "Search Tournament" } </button>
                </form>
            </div>        
        </Layout>
    }
}

#[function_component(TournamentView)]
fn tournament_view(props: &TournamentViewProps) -> Html {
    // Send a request to the backend for the tournament info
    let tournament: UseStateHandle<Option<Tournament>> = use_state(|| None);

    // None = loading, Some(true) = exists, Some(false) = not found
    let exists: UseStateHandle<Option<bool>> = use_state(|| None);

    // Ensure that tournament exists
    {
        let code = props.code.clone();
        let exists = exists.clone();

        use_effect_with(
            (),
            move |_| {
                // Remember to always move clones into async blocks!
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::post("http://localhost:2000/exists_tournament")
                        .header("Content-Type", "application/json")
                        .body(code).unwrap()
                        .send().await.expect("Failed to send get request");

                    exists.set(resp.json().await.expect("Failed to get bool"));
                });
            }
        );
    }
    
    {
        let code = props.code.clone();
        let tournament = tournament.clone();

        use_effect_with(
            (),
            move |_| {
                // Remember to always move clones into async blocks!
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::post("http://localhost:2000/get_tournament")
                        .header("Content-Type", "application/json")
                        .body(code).unwrap()
                        .send().await.expect("Failed to send get request");

                    tournament.set(resp.json().await.expect("Failed to deserialize"));
                });
            }
        )
    }

    // Create the html for the tournament before the actual html! macro
    // TODO:
    // Update scores to the backend WIP
    // Put more tournament info, not only FixtureMakerInput
    // I hate myself

    let tournament_html = {
        html! {
            { for (*tournament).clone().unwrap().matches.iter().map(|(date_idx, games)| html! {
                <div>
                    <h3>{ format!("Date {}", date_idx + 1) }</h3>
                    <ul>
                        { for games.iter().enumerate().map(|(game_idx, game)| html! {
                            <tr>
                                <td>{ &game.home_team.name }</td>
                                <td>
                                    <input
                                        type="number"
                                        min="0"
                                        max="100"
                                        value={ game.home.to_string() }
                                        oninput={ 
                                            let code = props.code.clone();
                                            score_input(
                                                tournament.clone(), 
                                                *date_idx, game_idx, TeamRole::Home, 
                                                code,
                                            ) 
                                        }
                                    />
                                </td>
                                <td>{ "vs" }</td>
                                <td>
                                    <input
                                        type="number"
                                        min="0"
                                        max="100"
                                        value={game.away.to_string()}
                                        oninput={ 
                                            let code = props.code.clone();
                                            score_input(
                                                tournament.clone(), 
                                                *date_idx, game_idx, TeamRole::Away,
                                                code,
                                            ) 
                                        }
                                    />
                                </td>
                                <td>{ &game.away_team.name }</td>
                            </tr>
                        })}
                    </ul>
                </div>
                })
            }
        }
    };

    // TODO: Make an enum that does this logic, for better understanding
    // NotLoaded
    // NotFound
    // Found(Tournament) 
    // maybe await for the tournament_html to finish or something
    // im not really that familiar with async and web dev yet 
    // but this works fine so it's cool
    match *exists {
        None => html!{ "Loading.. Please Wait" },
        Some(false) => html! { "ERROR 404: Tournament Not Found" },
        Some(true) => { 
            if let Some(t) = Some(tournament_html) {
                html! {
                    <Layout title="Tournament">
                        { t }
                    </Layout>
                }
            } else {
                html!{ 
                    <Layout title="Tournament">
                        { "Loading.. Please Wait" }
                    </Layout>
                }
            }
        }
    }
}

fn score_input(
    tournament: UseStateHandle<Option<Tournament>>,
    date_idx: usize,
    game_idx: usize,
    team_role: TeamRole,
    code: String,
) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| { // How does e get here idk but it works
        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
        let score: i32 = parse_score_input(input);
        
        let code = code.clone();

        let mut new_fixture = (*tournament).clone();    // mut keyword here is important
        if let Some(ref mut fix) = new_fixture {        // because we mutate next
            // Update scores
            let mut games = fix.matches[&date_idx].clone();
            let game = &mut games[game_idx];

            match team_role {
                TeamRole::Home => game.home = score,
                TeamRole::Away => game.away = score,
            }

            // Update backend
            let new_match = UpdateMatch {
                code,
                game_idx: game_idx as i32,
                date_idx: date_idx as i32,
                home: game.home,
                away: game.away,
            };

            wasm_bindgen_futures::spawn_local(async move {
                Request::post("http://localhost:2000/update_match")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&new_match).unwrap())
                    .unwrap().send().await.expect("Failed to send get request");
            });

            // Update frontend
            tournament.set(Some(fix.clone()));
        }
    })
}

fn parse_score_input(score_input: web_sys::HtmlInputElement) -> i32 {
    let score: i32 = score_input.value().parse().unwrap_or(0);

    if (0..=100).contains(&score) { // Check that score is in range
        score
    } else {
        0
    }
}

#[function_component(Dev)]
fn dev() -> Html {
    let password = use_state(|| "".to_string());
    let result = use_state(|| "".to_string());

    let onsubmit = {
        let password = password.clone();
        let result = result.clone();

        Callback::from(move |e: yew::SubmitEvent| {
            e.prevent_default();
            
            let password = password.clone();
            let result = result.clone();

            if window().unwrap().confirm_with_message(
                " WARNING: DELETING ALL DATA IN THE DATABASE, ARE YOU SURE?"
            ).unwrap_or(false) {
                // Send post to the backend to nuke the database
                wasm_bindgen_futures::spawn_local(async move {
                    result.set(
                        Request::post("http://localhost:2000/reset_database")
                            .header("Content-Type", "application/json")
                            .body(&*password)
                            .unwrap().send().await
                            .expect("Password Failed")
                            .text().await.unwrap()
                    )                
                }); 
            }
        })
    };

    html! { 
        <div>
            { "Dev page for DB reset" }
            <form { onsubmit }> // onsubmit={onsubmit}, could also do something like 
                                // onsubmit={ custom_name }
                <input type="password" placeholder="Secret Password"
                    value={ (*password).clone() }
                    // This is kinda lame having to do the updating the value myself,
                    // the yew crate might want to make this a bit simpler :D
                    oninput={ Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        password.set(input.value());
                    })}
                />
                <button type="submit" > { "DELETE ALL" } </button>
            </form>
            { (*result).clone() }
        </div> 
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

// Model to the data we want to get from form 
// And then send to the backend
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)] // For handling as JSON 
pub struct FixtureMakerInput {
    pub tournament_name: String,
    pub team_number: usize,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct UpdateMatch {
    code: String,
    game_idx: i32,
    date_idx: i32,
    home: i32,
    away: i32,
}

pub enum TeamRole {
    Home,
    Away,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Team {
    pub name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub game_idx: i32,
    pub date_idx: i32,
    pub home_team: Team,
    pub away_team: Team,
    pub home: i32,
    pub away: i32,
}
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Date {
    pub games: Vec<Game>,
    pub date_idx: usize,
}
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Tournament {
    pub name: String,
    pub teams: Vec<Team>,
    pub matches: BTreeMap<usize, Vec<Game>>
}
