use yew::prelude::*;
use yew_oauth2::oauth2::*;
use yew_oauth2::prelude::*;

//REDIRECT_URL='http://localhost:8080/auth'

#[function_component(App)]
fn app() -> Html {
    let config = Config {
        client_id: "345272141598641".into(),
        auth_url: "https://www.facebook.com/v18.0/dialog/oauth".into(),
        //token_url: "https://graph.facebook.com/v18.0/oauth/access_token".into(),
        token_url: "https://localhost:8080/auth/token".into(),
    };

    html! {
        <>
            <h1>{ "Hello, Cruel World!" }</h1>
            <OAuth2
                {config}
            >
                <AuthApp/>
            </OAuth2>
        </>
    }
}

#[function_component(AuthApp)]
fn auth_app() -> Html {
    let agent = use_auth_agent().expect("Must be in an oauth tag");

    let login = {
        let agent = agent.clone();
        Callback::from(move |_| {
            let _ = agent.start_login();
        })
    };

    let logout = Callback::from(move |_| {
        let _ = agent.logout();
    });

    html!(
        <>
            <Failure><FailureMessage/></Failure>
            <Authenticated>
                { "Welcome" }
                <button onclick={logout}>{ "Logout" }</button>
            </Authenticated>
            <NotAuthenticated>
                { "Please Login" }
                <button onclick={login}>{ "Login" }</button>
            </NotAuthenticated>
        </>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
