#[derive(Clone)]
pub struct Ticker {
    pub name: String,
    pub description: String,
    pub actions: i32,
}

pub struct User {
    pub name: String,
    pub nicho_coins: i32,
}
