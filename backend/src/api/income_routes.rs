use rocket::State;
use rocket::serde::json::{Json};
use rocket::http::Status;
use rocket::response::status;

use sqlx::{Pool, MySql};

use shared::models::income::{Income, IncomeCreate};
use crate::db::income_repository::{
    update_income as update_income_db,
    create_income as create_income_db,
    get_income_by_user_id as get_income_by_user_id_db,
    delete_income as delete_income_db,
};


// CREATE income route
#[post("/income", data = "<income>")]
pub async fn create_income_route(
    pool: &State<Pool<MySql>>,
    income: Json<IncomeCreate>,
) -> Result<status::Created<Json<Income>>, status::BadRequest<String>> {
    match create_income_db(&pool, &income.0).await {
        Ok(income_id) => {
            let created_income = Income {
                id: income_id as u32,
                user_id: income.user_id,
                amount: income.amount,
                name: income.name.clone(),
                description: income.description.clone(),
            };
            let response = status::Created::new(format!("/income/{}", income_id)).body(Json(created_income));
            Ok(response)
        }
        Err(e) => Err(status::BadRequest(Some(e.to_string()))),
    }
}


// READ income by user_id route
#[get("/income/user/<user_id>")]
pub async fn get_income_by_user_id_route(
    pool: &State<Pool<MySql>>,
    user_id: u32,
) -> Result<Json<Vec<Income>>, Status> {
    
    match get_income_by_user_id_db(&pool, user_id).await {
        Ok(incomes) => Ok(Json(incomes)),
        Err(e) => {
            println!("Error: {}", e);
            Err(Status::InternalServerError)
        }
    }
}


// UPDATE income route
#[put("/income/<income_id>", data = "<updated_income>")]
pub async fn update_income_route(
    pool: &State<Pool<MySql>>,
    income_id: u32,
    updated_income: Json<Income>,
) -> Result<Json<Income>, status::BadRequest<String>> {
    
    let mut updated_income_data = updated_income.0;
    updated_income_data.id = income_id;

    match update_income_db(&pool, &updated_income_data).await {
        Ok(_) => Ok(Json(updated_income_data)),
        Err(e) => Err(status::BadRequest(Some(e.to_string()))),
    }
}


// DELETE income route
#[delete("/income/<income_id>")]
pub async fn delete_income_route(
    pool: &State<Pool<MySql>>,
    income_id: u32,
) -> Result<status::NoContent, status::BadRequest<String>> {
    
    match delete_income_db(&pool, income_id.into()).await {
        Ok(_) => Ok(status::NoContent),
        Err(e) => Err(status::BadRequest(Some(e.to_string()))),
    }
}


// OPTIONS income route
#[options("/api/income")]
pub fn options_income_route() -> Status {
    Status::Ok
}
