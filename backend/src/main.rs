#[macro_use]
extern crate rocket;

mod db {
    pub mod connection;
    pub mod user_repository;
    pub mod income_repository;
}

mod api {
    pub mod user_routes;
    pub mod income_routes;
}

mod middleware {
    pub mod cors;
}

use rocket::fs::FileServer;
use dotenvy::dotenv;
use crate::db::connection;
use crate::api::user_routes::{
    create_user,
    get_user_by_id,
    update_user_email,
    delete_user_by_id,
};
use crate::api::income_routes::{
    create_income_route,
    get_income_by_user_id_route,
    update_income_route,
    delete_income_route,
    options_income_route
};
use crate::middleware::cors::CORS;
// use crate::middleware::auth::Auth;


#[launch]
async fn rocket() -> _ {

    dotenv().ok();

    let pool = connection::pool().await;
    rocket::build()
        .attach(CORS)
        .manage(pool)
        .mount("/", FileServer::from("../frontend/dist"))
        .mount(
            "/api",
            routes![
                // User routes
                create_user,
                get_user_by_id,
                update_user_email,
                delete_user_by_id,
                // Income routes
                create_income_route,
                get_income_by_user_id_route,
                update_income_route,
                delete_income_route,
                options_income_route,
            ],
        )
}