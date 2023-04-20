use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserCreate {
    pub username: String,
}

// impl UserCreate {
//     pub fn new(username: String, email: String) -> Self {
//         UserCreate { username, email }
//     }
// }
