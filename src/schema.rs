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

    pub author: i32,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub nicho_coins: i32,
    pub portfolio: HashMap<i32, i32>,
    #[serde(skip_serializing)]
    pub token: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub author: i32,
    pub date: i64,
    pub transaction_type: TransactionType,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TransactionType {
    Creation,
    Buy(i32),
    Sell(i32),
}
