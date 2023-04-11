use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
}

// impl UserCreate {
//     pub fn new(username: String, email: String) -> Self {
//         UserCreate { username, email }
//     }
// }
