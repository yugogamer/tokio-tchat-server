use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  Message {
    pub message : String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username : String,
}