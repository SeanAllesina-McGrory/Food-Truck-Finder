use axum::response::IntoResponse;
use axum::Form;
use axum::Json;
use dotenv::dotenv;
use log::{error, info};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenBody {
    grant_type: String,
    code: String,
    code_verifier: String,
    redirect_uri: String,
}

pub async fn token(Form(body): Form<TokenBody>) -> impl IntoResponse {
    dotenv().ok();
    let client_secret = match env::var("TEST_CLIENT_SECRET") {
        Err(e) => panic!("{e}"),
        Ok(client_secret) => client_secret,
    };
    info!("Hello {}", client_secret.clone());
    let req: TokenBody = body.into();
    println!("{:?}", req);
    let client = BasicClient::new(
        ClientId::new("345272141598641".to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        AuthUrl::new("https://www.facebook.com/v18.0/dialog/oauth".to_string()).expect("blah"),
        Some(
            TokenUrl::new("https://graph.facebook.com/v18.0/oauth/access_token".to_string())
                .expect("blah"),
        ),
    )
    .set_redirect_uri(
        RedirectUrl::new(req.redirect_uri.clone()).expect("Issue constructing Redirect url"),
    );

    let pkce_verifier = PkceCodeVerifier::new(req.code_verifier.clone());
    let token_result = client
        .exchange_code(AuthorizationCode::new(req.code.clone()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await;

    match token_result {
        Err(err) => {
            error!("{:?}", err.to_string());
            panic!("TODO better error handling here");
        }
        Ok(val) => {
            info!("Tokens received from OAuth provider!");
            Json(val)
        }
    }
}
