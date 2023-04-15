use rocket::serde::json::{Json, Value};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use rocket::http::Status;
use rocket::response::status;
use reqwest::Client;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use sqlx::{Pool, MySql};
use rocket::State;

use uuid::Uuid;

use crate::middleware::auth:: { fetch_and_store_google_public_keys };
use crate::db::user_repository::{ find_or_create_user, create_session };

#[derive(Deserialize)]
pub struct AuthCode {
    code: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    access_token: String,
    session_id: String,
}

#[derive(Debug, Deserialize)]
struct GoogleIdTokenClaims {
    _sub: String,
    email: String,
    email_verified: bool,
}

#[post("/handle_auth_code", data = "<auth_code>")]
pub async fn handle_auth_code(
    pool: &State<&Pool<MySql>>,
    auth_code: Json<AuthCode>
) -> Result<Json<AuthResponse>, status::Custom<Value>> {
    let client_id = env::var("GOOGLE_CLIENT_ID").unwrap();
    let client_secret = env::var("GOOGLE_CLIENT_SECRET").unwrap();
    let redirect_uri = env::var("REDIRECT_URI").unwrap();

    let client = Client::new();
    let token_request = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("client_id", &client_id),
            ("client_secret", &client_secret),
            ("code", &auth_code.code),
            ("redirect_uri", &redirect_uri),
            ("grant_type", &"authorization_code".to_owned()),
        ]);

    let token_response = token_request.send().await.map_err(|e| {
        status::Custom(
            Status::InternalServerError,
            json!({ "error": format!("Token request error: {:?}", e) }),
        )
    })?;

    if token_response.status().is_success() {
        let token_data: Value = token_response
        .json()
        .await
        .map_err(|e| {
            status::Custom(
                Status::InternalServerError,
                json!({ "error": format!("Failed to parse JSON: {:?}", e) }),
            )
        })?;

        let access_token = token_data["access_token"].as_str().unwrap().to_string();
        let id_token = token_data["id_token"].as_str().unwrap().to_string();

        let google_id_token_claims = decode_google_id_token(&id_token).await;
        // Check if the email is verified
        if !google_id_token_claims.email_verified {
            return Err(status::Custom(
                Status::Unauthorized,
                json!({ "error": "Email is not verified" }),
            ));
        }

        let user = find_or_create_user(pool.inner(), &google_id_token_claims.email).await.map_err(|e| {
            status::Custom(
                Status::InternalServerError,
                json!({ "error": format!("Failed to find or create user: {:?}", e) }),
            )
        })?;

        let session_id = create_session(pool.inner(), user.id).await.map_err(|e| {
            status::Custom(
                Status::InternalServerError,
                json!({ "error": format!("Failed to create session: {:?}", e) }),
            )
        })?;

        Ok(Json(AuthResponse { access_token, session_id }))

    } else {
        Err(status::Custom(
            Status::BadRequest,
            json!({ "error": format!("Request error: {:?}", token_response) }),
        ))
    }
}

async fn decode_google_id_token(id_token: &str) -> GoogleIdTokenClaims {
    let validation = Validation {
        iss: Some("accounts.google.com".to_string()),
        algorithms: vec![Algorithm::RS256],
        ..Validation::default()
    };

    let kid = match jsonwebtoken::decode_header(id_token) {
        Ok(header) => header.kid.unwrap_or_default(),
        Err(_) => panic!("Invalid ID token"),
    };

    let google_public_keys = fetch_and_store_google_public_keys().await;

    let (public_key_n, public_key_e) = match google_public_keys.get(&kid) {
        Some((n, e)) => (n, e),
        None => panic!("Invalid ID token"),
    };

    match decode::<GoogleIdTokenClaims>(
        id_token,
        &DecodingKey::from_rsa_components(&public_key_n, &public_key_e),
        &validation,
    ) {
        Ok(claims) => claims.claims,
        Err(_) => panic!("Invalid ID token"),
    }
}