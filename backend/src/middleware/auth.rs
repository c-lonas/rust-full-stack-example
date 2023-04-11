use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use rocket::{Request};
use rocket::request::{self, FromRequest, Outcome};
use rocket::http::Status;
use serde::{ Deserialize };
use async_trait::async_trait;
use std::collections::HashMap;

pub struct Auth;

#[derive(Debug, Deserialize)]
struct Claims {
    // sub: String,
    // exp: usize,
}

#[async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let id_token = match request.headers().get_one("Authorization") {
            Some(token) if token.starts_with("Bearer ") => token[7..].to_string(),
            _ => return Outcome::Failure((Status::Unauthorized, ())),
        };

        let validation = Validation {
            iss: Some("accounts.google.com".to_string()),
            algorithms: vec![Algorithm::RS256],
            ..Validation::default()
        };

        let kid = match jsonwebtoken::decode_header(&id_token) {
            Ok(header) => header.kid.unwrap_or_default(),
            Err(_) => return Outcome::Failure((Status::Unauthorized, ())),
        };

        let google_public_keys = fetch_and_store_google_public_keys().await;

        let (public_key_n, public_key_e) = match google_public_keys.get(&kid) {
            Some((n, e)) => (n, e),
            None => return Outcome::Failure((Status::Unauthorized, ())),
        };

        match decode::<Claims>(
            &id_token,
            &DecodingKey::from_rsa_components(&public_key_n, &public_key_e),
            &validation,
        ) {
            Ok(_) => Outcome::Success(Auth),
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

pub async fn fetch_and_store_google_public_keys() -> HashMap<String, (String, String)> {
    let url = "https://www.googleapis.com/oauth2/v3/certs";
    let public_keys_json: serde_json::Value = reqwest::get(url).await.unwrap().json().await.unwrap();
    let public_keys = public_keys_json["keys"].as_array().unwrap();
    let mut keys_map = HashMap::new();

    for key in public_keys {
        let n = key["n"].as_str().unwrap().to_string();
        let e = key["e"].as_str().unwrap().to_string();
        let kid = key["kid"].as_str().unwrap().to_string();
        keys_map.insert(kid, (n, e));
    }

    keys_map
}
