use serde::{Serialize, Deserialize};
use std::default::Default;

#[derive(Debug, Serialize, Deserialize)]
pub struct Income {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub amount: u32,
    pub user_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncomeCreate {
    pub name: String,
    pub description: Option<String>,
    pub amount: u32,
    pub user_id: u32,
}



impl Default for IncomeCreate {
    fn default() -> Self {
        IncomeCreate {
            name: String::from(""),
            description: None,
            amount: 0,
            user_id: 0,
        }
    }
}