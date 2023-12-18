use gloo::console::log;
use reqwasm::http::Request;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty};
use std::env;
use yew::prelude::*;

#[derive(Serialize)]
pub struct Auth {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vendor {
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub auth_token: Option<String>,
    pub description: Option<String>,
    pub vendor_type: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub website: Option<String>,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Auth {
        token: String::from("1234"),
    });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:8081/v1/vendors")
                    .header("", &state.token)
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<Vendor>>()
                    .await
                    .unwrap();

                log!(to_string_pretty(&response).unwrap());
            });
        })
    };

    html! {
        <>
            <h1>{ "Hello, Cruel World!" }</h1>
            <button {onclick}>{"get vendors"}</button>
            <a class="button" href={format!("https://www.facebook.com/v18.0/dialog/oauth?client_id=345272141598641&&redirect_uri=http://localhost:8080/auth&state={}", json!(*state))} aria-label="Learn about the HTML button tag"><span>{"HTML button tag"}</span></a>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
