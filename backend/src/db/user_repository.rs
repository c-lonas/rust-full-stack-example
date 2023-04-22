use sqlx::{Pool, MySql};
use sqlx::mysql::{MySqlRow};
use sqlx::Row;
use shared::models::user::{User, UserCreate};

pub async fn get_all_users(pool: &Pool<MySql>) -> Result<Vec<User>, sqlx::Error> {
    let users: Vec<User> = sqlx::query("SELECT  id, username FROM users")
        .map(|row: MySqlRow| {
            User {
                id: row.get("id"),
                username: row.get("username"),
            }
        })
        .fetch_all(pool)
        .await?;
        
    Ok(users)
}


pub async fn create_user(pool: &Pool<MySql>, user: &UserCreate) -> Result<u64, sqlx::Error> {

    let result = sqlx::query!(
        "INSERT INTO users (username) VALUES (?)",
        &user.username,
    )
    .execute(pool)
    .await?;
        

    Ok(result.last_insert_id())
}


pub async fn get_user_by_id(pool: &Pool<MySql>, user_id: u32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(User,
        "SELECT id, username FROM users WHERE id = ?",
        user_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}



pub async fn delete_user_by_id(pool: &Pool<MySql>, user_id: u32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM users WHERE id = ?", user_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}



// pub async fn find_or_create_user(pool: &Pool<MySql> ) -> Result<User, sqlx::Error> {
//     let maybe_user = sqlx::query_as!(User, "SELECT id, username FROM users WHERE email = ?", email)
//         .fetch_optional(pool)
//         .await?;

//     if let Some(user) = maybe_user {
//         Ok(user)
//     } else {
//         let user_create = UserCreate {
//             username: email.split("@").next().unwrap_or("").to_string(),
//         };

//         let user_id = create_user(pool, &user_create).await?;
//         let created_user = get_user_by_id(pool, user_id as u32).await?;
//         Ok(created_user)
//     }
// }



