use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Serialize, Deserialize};
use web_sys::window;

// TODO:
// Make page pretty

#[function_component]
fn App() -> Html {
    // Variables to store and update local component state
    // holding the current value of the input field
    let tournament = use_state(|| "".to_string());
    let team_number: UseStateHandle<i32> = use_state(|| 0); 
    let fixture: UseStateHandle<Option<Fixture>> = use_state(|| None);

    // Called when form is submited via button type="submit"
    let onsubmit = {
        // Cloning state handlers to use inside closure
        let tournament = tournament.clone();
        let team_number = team_number.clone();
        let fixture = fixture.clone();
        
        // TODO:

        Callback::from(move |e: yew::SubmitEvent|{
            e.prevent_default(); // Prevent page reloading and Networks errors when posting

            // Collect input values to FixtureMakerInput struct
            let input = FixtureMakerInput { 
                tournament: (*tournament).clone(),
                team_number: (*team_number),
            };

            // Move fixture here :D
            let fixture = fixture.clone();

            // Copilot says its async because it runs inside the JS event loop in the browser
            // Imma pretend I understand that
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::post("http://localhost:3000/fixture")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&input).unwrap()) // Serialize to JSON
                    .unwrap()
                    .send()
                    .await
                    .expect("Failed to send request");

                // Read response
                let fix = resp.json().await.expect("Failed to deserialize response");
                fixture.set(Some(fix));

                // Log in console
                // web_sys::console::log_1(&format!("{:?}", fix).into());
            });  
        })
    };

    let reset = {
        // Clone handles to use inside the closure
        let tournament = tournament.clone();
        let team_number = team_number.clone();
        let fixture = fixture.clone();

        Callback::from(move |_| {
            // TODO: Update this to be a custom modal rather than a basic confirm dialog
            if window().unwrap().confirm_with_message("Sure you want to reset? ").unwrap_or(false) {
                tournament.set("".to_string());
                team_number.set(0);
                fixture.set(None);
            }
        })
    };

    // Create the html for the fixture before the actual html! macro
    let fixture_html = (*fixture).as_ref().map(|fix| fix.dates.iter().enumerate().map(|(date_idx, date)| {
        html! {
            <div>
                <h3>{ format!("Date {}", date_idx + 1) }</h3>
                // TODO:
                // Future-proofing - store data
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
                                        let fixture_handle = fixture.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                            let value = input.value();
                                            let mut new_fixture = (*fixture_handle).clone();
                                            if let Some(ref mut fix) = new_fixture {
                                                let game = &mut fix.dates[date_idx].games[game_idx];
                                                game.home = value.parse().unwrap_or(0);
                                                fixture_handle.set(Some(fix.clone()));
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
                                        let fixture_handle = fixture.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                            let value = input.value();
                                            let mut new_fixture = (*fixture_handle).clone();
                                            if let Some(ref mut fix) = new_fixture {
                                                let game = &mut fix.dates[date_idx].games[game_idx];
                                                game.away = value.parse().unwrap_or(0);
                                                fixture_handle.set(Some(fix.clone()));
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

            // Reset button
            <button type="button" onclick={reset}> { "Reset" } </button>

            { // Display the fixture :D
                if let Some(html) = fixture_html {
                    html
                } else {
                    html! {}
                }
            }
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
    pub tournament: String,
    pub team_number: i32,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Team {
    pub name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub home_team: Team,
    pub away_team: Team,
    pub home: i32,
    pub away: i32,
}
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Date {
    pub games: Vec<Game>,
}
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Fixture {
    pub teams: Vec<Team>,
    pub dates: Vec<Date>,
}
