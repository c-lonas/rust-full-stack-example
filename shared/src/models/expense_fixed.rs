use serde::{Serialize, Deserialize};
use std::default::Default;


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Interval {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenseFixed {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub amount: u32,
    pub interval: Interval,
    pub user_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpenseFixedCreate {
    pub name: String,
    pub description: Option<String>,
    pub amount: u32,
    pub interval: Interval,
    pub user_id: u32,
}


impl Default for ExpenseFixedCreate {
    fn default() -> Self {
        ExpenseFixedCreate {
            name: String::from(""),
            description: None,
            amount: 0,
            interval: Interval::Monthly,
            user_id: 0,
        }
    }
}