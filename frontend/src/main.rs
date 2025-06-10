use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Serialize, Deserialize};

// Model to the data we want to get from form 
// And then send to the backend
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)] // For handling as JSON 
pub struct FixtureMakerInput {
    pub tournament: String,
    pub team_number: String,
}

#[function_component]
fn App() -> Html {
    // Variables to store and update local component state
    // holding the current value of the input field
    let tournament = use_state(|| "".to_string());
    let team_number = use_state(|| "".to_string()); // *TODO I will prob want the team number field
                                                    // to be an int. Will figure out later, not so
                                                    // important now

    // Called when form is submited via button type="submit"
    let onsubmit = {
        // Cloning state handlers to use inside closure
        let tournament = tournament.clone();
        let team_number = team_number.clone();

        Callback::from(move |_|{
            // Collect input values to FixtureMakerInput struct
            let input = FixtureMakerInput { 
                tournament: (*tournament).clone(),
                team_number: (*team_number).clone(),
            };

            // Copilot says its async because it runs inside the JS event loop in the browser
            // Imma pretend I understand that
            wasm_bindgen_futures::spawn_local(async move {
                // Make a POST Request to the backend endpoint (/fixture *TODO)
                let resp = Request::post("http://localhost:3000/fixture")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&input).unwrap()) // Serialize to JSON
                    .unwrap()
                    .send()
                    .await
                    .expect("Failed to send request");

                // Read and log response
                // *TODO display the fixture (not yet) recieved in UI
                let text = resp.text().await.expect("Failed to read response");
                web_sys::console::log_1(&text.into());
            });  
        })
    };

    html! {
        <div>
            <form {onsubmit}>
                <input type="text" placeholder="Tournament Name"
                    value={(*tournament).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        tournament.set(input.value());
                    })}
                />
                <input type="number" placeholder="Number of Teams"
                    value={(*team_number).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        team_number.set(input.value());
                    })}
                />
                <button type="submit">{ "Create Fixture" }</button>
            </form>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
