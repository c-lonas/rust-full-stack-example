use sqlx::{Pool, MySql};
use sqlx::mysql::{MySqlRow};
use sqlx::Row;
use shared::models::user::{User, UserCreate};
use uuid::Uuid;

pub struct Session {
    pub user_id: u32,
    pub session_id: String,
}


pub async fn _get_all_users(pool: &Pool<MySql>) -> Result<Vec<User>, sqlx::Error> {
    let users: Vec<User> = sqlx::query("SELECT  id, username, email FROM users")
        .map(|row: MySqlRow| {
            User {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
            }
        })
        .fetch_all(pool)
        .await?;
        
    Ok(users)
}


pub async fn create_user(pool: &Pool<MySql>, user: &UserCreate) -> Result<u64, sqlx::Error> {

    let result = sqlx::query!(
        "INSERT INTO users (username, email) VALUES (?, ?)",
        &user.username,
        &user.email,
    )
    .execute(pool)
    .await?;
        

    Ok(result.last_insert_id())
}


pub async fn get_user_by_id(pool: &Pool<MySql>, user_id: u32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(User,
        "SELECT id, username, email FROM users WHERE id = ?",
        user_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}


pub async fn update_user_email(pool: &Pool<MySql>, user_id: u32, new_email: &str) -> Result<User, sqlx::Error> {
    sqlx::query!(
        "UPDATE users SET email = ? WHERE id = ?",
        new_email,
        user_id,
    )
    .execute(pool)
    .await?;

    get_user_by_id(pool, user_id).await
}


pub async fn delete_user_by_id(pool: &Pool<MySql>, user_id: u32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM users WHERE id = ?", user_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}



pub async fn find_or_create_user(pool: &Pool<MySql>, email: &str) -> Result<User, sqlx::Error> {
    let maybe_user = sqlx::query_as!(User, "SELECT id, username, email FROM users WHERE email = ?", email)
        .fetch_optional(pool)
        .await?;

    if let Some(user) = maybe_user {
        Ok(user)
    } else {
        let user_create = UserCreate {
            username: email.split("@").next().unwrap_or("").to_string(),
            email: email.to_string(),
        };

        let user_id = create_user(pool, &user_create).await?;
        let created_user = get_user_by_id(pool, user_id as u32).await?;
        Ok(created_user)
    }
}



