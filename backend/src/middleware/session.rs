use reqwest::get;
use rocket::{Request};
use rocket::request::{self, FromRequest, Outcome};
use rocket::http::Status;
use async_trait::async_trait;
use sqlx::{Pool, MySql};
use rocket::State;

use crate::db::user_repository::{Session, get_session};



pub struct SessionAuth {
    pub user_id: i32,
}


#[async_trait]
impl<'r> FromRequest<'r> for SessionAuth {
    type Error = ();

    async fn from_request<'r>(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let session_token = match request.headers().get_one("Session-Token") {
            Some(token) => token,
            _ => return Outcome::Failure((Status::Unauthorized, ())),
        };

        let pool = request.guard::<&State<Pool<MySql>>>().await.expect("database pool not found");
        let session = match get_session(&pool, session_token).await {
            Ok(s) => s,
            Err(_) => return Outcome::Failure((Status::Unauthorized, ())),
        };

        Outcome::Success(SessionAuth {
            user_id: session.user_id,
        })
    }
}

