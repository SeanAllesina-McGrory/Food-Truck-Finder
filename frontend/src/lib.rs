use gloo::console::log;
use gloo_net::http;
use yew::prelude::*;
use yew_oauth2::oauth2::*;
use yew_oauth2::prelude::*;

//REDIRECT_URL='http://localhost:8080/auth'

#[function_component(App)]
pub fn app() -> Html {
    let config = Config {
        client_id: "345272141598641".into(),
        auth_url: "https://www.facebook.com/v18.0/dialog/oauth".into(),
        //token_url: "https://graph.facebook.com/v18.0/oauth/access_token".into(),
        token_url: "http://localhost:3443/auth/token".into(),
    };

    html! {
        <>
            <h1>{ "Hello, Cruel World!" }</h1>
            <OAuth2
                {config}
                scopes={vec!["public_profile".to_string(),"email".to_string()]}
            >
                <AuthApp/>
            </OAuth2>
        </>
    }
}

#[function_component(AuthApp)]
fn auth_app() -> Html {
    let agent = use_auth_agent().expect("Must be in an oauth tag");
    let token = use_latest_access_token().unwrap();

    let login = {
        let agent = agent.clone();
        Callback::from(move |_| {
            let _ = agent.start_login();
        })
    };

    let logout = Callback::from(move |_| {
        let _ = agent.logout();
    });

    let auth = use_context::<OAuth2Context>();
    //let a = &auth.clone().unwrap().authentication().unwrap().access_token;

    html!(
        <>
            <Failure><FailureMessage/></Failure>
            <Authenticated>
                { "Welcome" }
                { auth.expect("").access_token() }
                { token.access_token() }
                <button onclick={logout}>{ "Logout" }</button>
                <a href="http://localhost:3443/api/vendors">{ "Vendors" }</a>
            </Authenticated>
            <NotAuthenticated>
                { "Please Login" }
                <button onclick={login}>{ "Login" }</button>
            </NotAuthenticated>
        </>
    )
}
