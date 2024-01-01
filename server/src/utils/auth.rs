use crate::server::state;
use axum::extract::State;
use axum::Form;
use axum::Json;
use color_eyre::{eyre::bail, Result};
use dotenv::dotenv;
use log::info;
use oauth2::EmptyExtraTokenFields;
use oauth2::{
    basic::{BasicClient, BasicErrorResponse, BasicTokenType},
    reqwest::async_http_client,
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, PkceCodeVerifier, RedirectUrl,
    StandardTokenResponse, TokenResponse, TokenUrl,
};

use serde::{Deserialize, Serialize};
use std::env;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use crate::database::models::{Record, Vendor};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenBody {
    grant_type: String,
    code: String,
    code_verifier: String,
    redirect_uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct UserInfo {
    name: String,
    id: String,
}

pub async fn token(
    State(state): State<state::AppState>,
    Form(body): Form<TokenBody>,
) -> Json<oauth2::StandardTokenResponse<EmptyExtraTokenFields, oauth2::basic::BasicTokenType>> {
    let mut token = match get_token(body.into()).await {
        Some(token) => token,
        None => panic!("TODO better error handling"),
    };
    let user_info = match get_info(token.access_token().secret().as_str(), "email".as_ref()).await {
        Some(user_info) => user_info,
        None => panic!("TODO Better error handling here"),
    };

    let name = user_info.name.to_string();
    let id = user_info.id.to_string();

    let login_result = process_login(name, id, token.clone(), state.db).await;

    println!("{:?}", login_result);
    dbg!(&token);

    Json(token)

    //let record_option_result: Result<Option<Record>, surrealdb::Error> = state.db.create()
}

async fn get_token(
    req: TokenBody,
) -> Option<StandardTokenResponse<EmptyExtraTokenFields, oauth2::basic::BasicTokenType>> {
    dotenv().ok();
    let client_secret = match env::var("TEST_CLIENT_SECRET") {
        Err(e) => panic!("{e}"),
        Ok(client_secret) => client_secret,
    };
    let client = BasicClient::new(
        ClientId::new("345272141598641".to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        AuthUrl::new("https://auth".to_string()).expect("blah"),
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
        Ok(token) => Some(token),
        Err(err) => {
            info!("{:?}", err);
            None
        }
    }
}

// Due to the apps lack of verification of facebook we arent able to access user email
// Instead the user id will be used instead and emails will be added later
// TODO: Add email
async fn get_info(token: &str, feilds: &str) -> Option<UserInfo> {
    // This is arbitrary since the api will always return name and id
    let uri = format!(
        "https://graph.facebook.com/v18.0/me?feilds=name,{}&access_token={}",
        feilds,
        token.to_string()
    );

    let response_result = reqwest::get(uri).await;
    let user_info_result = match response_result {
        Ok(user_info_result) => user_info_result.json::<UserInfo>().await,
        Err(_) => return None,
    };

    let user_info = match user_info_result {
        Ok(user_info) => user_info,
        Err(_) => return None,
    };

    println!("{:?}", &user_info);
    Some(user_info)
}

async fn process_login(
    name: String,
    identifier: String,
    token: oauth2::StandardTokenResponse<EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    db: Surreal<Client>,
) -> color_eyre::eyre::Result<()> {
    let vendor_vec: Vec<Vendor> = db.select("vendors").await?;

    let vendor_vec: Vec<Vendor> = vendor_vec
        .iter()
        .map(|v| v.clone())
        .filter(|vendor| vendor.email == identifier)
        .collect();

    dbg!(&vendor_vec);

    let vendor = match vendor_vec.len() {
        0 => {
            let vendor = Vendor::new(name).email(identifier);
            let option_result: Option<Record> = db
                .create(("vendors", vendor.uuid.clone().into_owned()))
                .content(vendor.clone())
                .await?;
            match option_result {
                Some(_) => vendor,
                None => bail!("Failed to create user!"),
            }
        }
        1 => vendor_vec.first().unwrap().clone(), // HACK: this will always work since we precheck the len
        // But its not a great way to write code so fix later
        _ => panic!("Nonunique idenfifier found!\n{}", identifier),
    };

    println!("{:?}", vendor);

    Ok(())
}
