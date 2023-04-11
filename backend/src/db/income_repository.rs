
use sqlx::{Pool, MySql};
use sqlx::mysql::{MySqlRow};  
use sqlx::Row;                
use shared::income::{Income, IncomeCreate};


// CREATE new income
pub async fn create_income(pool: &Pool<MySql>, income: &IncomeCreate) -> Result<u64, sqlx::Error> {
  
  let result = sqlx::query!(
        "INSERT INTO income (user_id, amount, name, description) VALUES (?, ?, ?, ?)",
        &income.user_id, 
        &income.amount, 
        &income.name, 
        &income.description)
        .execute(pool)
        .await?;

   Ok(result.last_insert_id())
}


// READ income by user id
pub async fn get_income_by_user_id(pool: &Pool<MySql>, user_id: u32) -> Result<Vec<Income>, sqlx::Error> {
    
    // Use query! macro instead of query_as! macro because the income struct
    // has an optional value for the description field
    let incomes = sqlx::query("SELECT id, name, description, amount, user_id FROM income WHERE user_id = ?")
        .bind(user_id)
        .map(|row: MySqlRow| {
            Income {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                amount: row.get("amount"),
                user_id: row.get("user_id"),
            }
        })
        .fetch_all(pool)
        .await?;

    Ok(incomes)
}


// UPDATE income
pub async fn update_income(pool: &Pool<MySql>, income: &Income) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE income SET name = ?, description = ?, amount = ? WHERE id = ?",
        income.name, income.description, income.amount, income.id)
        .execute(pool)
        .await?;

    Ok(result.last_insert_id())
}


// DELETE income
pub async fn delete_income(pool: &Pool<MySql>, income_id: u64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM income WHERE id = ?",
        income_id)
        .execute(pool)
        .await?;

    Ok(result.last_insert_id())
}