use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Event, EventTarget};
use yew::prelude::*;
use weblog::console_log;

use crate::entities::user::{User, from_json };
//use serde_json::Result;

#[function_component(App)]
pub fn app() -> Html {

    let username_handle = use_state(|| "".to_string());
    let username = (*username_handle).clone();

    let username_change = {
        let username_handle = username_handle.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                username_handle.set(input.value() );
                console_log!(format!( "Username changed to: {}", input.value() ) );
                let new_user : User = User::new(input.value());
                console_log!(format!( "New user: {:?}", new_user ) );
                let new_user_json = new_user.to_json();
                match new_user_json {
                    Ok(json) => {
                        console_log!(format!( "(json): {}", json ) );
                        let user_from_json = from_json(&json);
                        console_log!(format!( "(from): {:?}", user_from_json ) );
                    },
                    Err(e) => {
                        console_log!(format!( "Error converting user to json: {}", e ) );
                    }
                }

            }

        })
    };

    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            <input type="text" value={username.clone()} onchange={username_change}/>
            <button >
                {format!( "Generate cards for {}!", username.clone() )}
            </button>
            
        </main>
    }
}
