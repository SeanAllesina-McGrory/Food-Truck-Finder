use axum::{
    extract::{Extension, Request},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
};

// Ensures the request only modifies data belonging to the provided vendor id
//      Since we already confirmed the token matches the vendor id provided
//      We can assume either the token was leaked or the user is authorized
pub async fn authorizer(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    if request.method() == axum::http::Method::GET {
        return Ok(next.run(request).await);
    }

    // TODO: add authorization functionality

    Ok(next.run(request).await)
}

// Authenticates the request to ensure the requester's token matches the given ID
// FIX: This solution will not work if the vendor does not already exist
pub async fn authenticator(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    if request.method() == axum::http::Method::GET {
        return Ok(next.run(request).await);
    }
    let token_option = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = match token_option {
        Some(token) => token,
        None => return Err(StatusCode::NOT_FOUND),
    };
    let vendor_id_option = request
        .headers()
        .get("x-user-id")
        .and_then(|vendor_id| vendor_id.to_str().ok());

    let vendor_id = if let Some(vendor_id) = vendor_id_option {
        vendor_id
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    // WARN: Vendor id is used only as a temporary way to authorize the user
    //      JWTs will be used in the production version
    if vendor_id == token {
        return Ok(next.run(request).await);
    }

    Err(StatusCode::NOT_FOUND)
}
