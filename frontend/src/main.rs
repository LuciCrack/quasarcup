use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use serde::{Serialize, Deserialize};

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
    }
}

// TODO: refractor modules
// move function components for each route
// to a different module or something

#[function_component(Home)]
fn home() -> Html {
    html! { <div>{ "Welcome! Choose create or open tournament." }</div> }
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

            // Copilot says its async because it runs inside the JS event loop in the browser
            // Imma pretend I understand that

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
                <button type="submit">{ "Create Fixture" }</button>
            </form>
        </div>
    }}

#[derive(Properties, PartialEq, Debug, Clone, Eq)]
struct TournamentViewProps {
    pub code: String,
}
#[function_component(TournamentView)]
fn tournament_view(props: &TournamentViewProps) -> Html {
    // Send a request to the backend for the tournament info
    let tournament: UseStateHandle<Option<Tournament>> = use_state(|| None);

    // None = not checked yet, Some(true) = exists, Some(false) = not found
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

    // FIXME: doesnt show not found when doesnt exist
    if let Some(exists) = *exists {
        if !exists {
            return html! { <div>{ "Tournament not found." }</div> }
        }
    };

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
    // Update scores to the backend
    // Put more tournament info, not only fixture
    let tournament_html = (*tournament).as_ref().map(|fix| fix.matches.iter().enumerate().map(|(date_idx, date)| {

        html! {
            <div>
                <h3>{ format!("Date {}", date_idx + 1) }</h3>
                <ul>
                    { for date.games.iter().enumerate().map(|(game_idx, game)| html! {
                        <tr>
                            <td>{ &game.home_team.name }</td>
                            <td>
                                <input
                                    type="number"
                                    min="0"
                                    max="100"
                                    value={ game.home.to_string() }
                                    oninput={ {
                                        let tournament = tournament.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                            let value = input.value();
                                            let mut new_fixture = (*tournament).clone();
                                            if let Some(ref mut fix) = new_fixture {
                                                let game = &mut fix.matches[date_idx].games[game_idx];
                                                game.home = value.parse().unwrap_or(0);
                                                tournament.set(Some(fix.clone()));
                                            }
                                        })
                                    } }
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
                                        let tournament = tournament.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                            let value = input.value();
                                            let mut new_fixture = (*tournament).clone();
                                            if let Some(ref mut fix) = new_fixture {
                                                let game = &mut fix.matches[date_idx].games[game_idx];
                                                game.away = value.parse().unwrap_or(0);
                                                tournament.set(Some(fix.clone()));
                                            }
                                        })
                                    }
                                />
                            </td>
                            <td>{ &game.away_team.name }</td>
                        </tr>
                    })}
                </ul>
            </div>
        }
    }).collect::<Html>());

    // FIXME: returns loading when tournament does not exists
    html!{
        <div>
            {
                if (*tournament).is_some() {
                    tournament_html
                } else {
                    Some(html! { "Loading .." })
                }
            }
        </div>
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
    pub matches: Vec<Date>
}
