use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  Message {
    Message(String)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username : String,
}