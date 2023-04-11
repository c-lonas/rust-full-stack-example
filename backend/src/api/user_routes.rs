use rocket::State;
use rocket::serde::json::{Json};
use rocket::http::Status;
use rocket::response::status;

use sqlx::{Pool, MySql};

use shared::user::{User, UserCreate};
use crate::db::user_repository::{
    create_user as create_user_db,
    get_user_by_id as get_user_by_id_db,
    update_user_email as update_user_email_db,
    delete_user_by_id as delete_user_by_id_db,
};


// CREATE new user
#[post("/users", data = "<user>")]
pub async fn create_user(
    pool: &State<Pool<MySql>>,
    user: Json<UserCreate>,
) -> Result<status::Created<Json<UserCreate>>, status::BadRequest<String>> {

    match create_user_db(pool.inner(), &user.0).await {
        Ok(_) => {
            let response = status::Created::new(format!("/users/{}", user.0.username)).body(Json(user.0));
            Ok(response)
        }
        Err(e) => Err(status::BadRequest(Some(e.to_string()))),
    }
}

// READ existing user
#[get("/users/<id>")]
pub async fn get_user_by_id(
    pool: &State<Pool<MySql>>,
    id: u32
) -> Result<Json<User>, status::BadRequest<String>> {
    
    match get_user_by_id_db(pool.inner(), id).await {
        Ok(user) => {
           Ok(Json(user)) 
        }
        Err(e) => Err(status::BadRequest(Some(e.to_string()))),
    }
}


// UPDATE email address for existing user
#[put("/users/<id>", data = "<new_user_email>")]
pub async fn update_user_email(
    pool: &State<Pool<MySql>>,
    id: u32,
    new_user_email: &str
) -> Result<status::Custom<Json<User>>, status::BadRequest<String>> {
    
    match update_user_email_db(pool.inner(), id, new_user_email).await {
        Ok(user) => {
            let response = status::Custom(Status::Ok, Json(user));
            Ok(response)
        }
        Err(e) => Err(status::BadRequest(Some(e.to_string()))),
    }
}


// DELETE existing user
#[delete("/users/<id>")]
pub async fn delete_user_by_id(
    pool: &State<Pool<MySql>>,
    id: u32
) -> Result<Status, status::BadRequest<String>> {
    
    match delete_user_by_id_db(pool.inner(), id).await {
        Ok(_) => Ok(Status::NoContent),
        Err(e) => Err(status::BadRequest(Some(e.to_string()))),
    }
}