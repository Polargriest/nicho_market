use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Ticker {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub actions: i32,
}

#[derive(Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub nicho_coins: i32,
}
