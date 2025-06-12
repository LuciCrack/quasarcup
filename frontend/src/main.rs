use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Serialize, Deserialize};

// Model to the data we want to get from form 
// And then send to the backend
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)] // For handling as JSON 
pub struct FixtureMakerInput {
    pub tournament: String,
    pub team_number: i32,
}

#[function_component]
fn App() -> Html {
    // Variables to store and update local component state
    // holding the current value of the input field
    let tournament = use_state(|| "".to_string());
    let team_number: UseStateHandle<i32> = use_state(|| "".to_string().parse().unwrap_or(0)); 

    // Called when form is submited via button type="submit"
    let onsubmit = {
        // Cloning state handlers to use inside closure
        let tournament = tournament.clone();
        let team_number = team_number.clone();

        Callback::from(move |e: yew::SubmitEvent|{
            e.prevent_default(); // Prevent page reloading and Networks errors when posting
            
            // Collect input values to FixtureMakerInput struct
            let input = FixtureMakerInput { 
                tournament: (*tournament).clone(),
                team_number: (*team_number),
            };

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

                // *TODO 
                // recieve and deserialize fixture
                // display the fixture in UI

                // Read and log response
                let text = resp.text().await.expect("Failed to read response");
                web_sys::console::log_1(&text.into());
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
                <input type="number" min=2 max=1000 placeholder="Number of Teams"
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
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
