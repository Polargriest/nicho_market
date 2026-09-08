//! Entidades que forman parte del dominio de la bolsa de valores de nichos. Elementos que
//! Market maneja van en este módulo.
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub actions: i32,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub nicho_coins: i32,
    pub portfolio: HashMap<i32, i32>,
    // This might be a bad idea, since you can see all the user's fields in /users
    pub token: String,
}
