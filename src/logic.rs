use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::schema::{Ticker, User};

const INCREASE_RATE: i32 = 10;
pub const BASE_PRICE: i32 = 100;

#[derive(Debug)]
pub enum ApiError {
    BuyError,
    UserNotFound(i32),
    TickerNotFound(i32),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_message) = match self {
            ApiError::BuyError => (
                StatusCode::BAD_REQUEST,
                "Error while buying actions".to_string(),
            ),
            ApiError::UserNotFound(id) => (
                StatusCode::BAD_REQUEST,
                format!("User with ID {id} not found."),
            ),
            ApiError::TickerNotFound(id) => (
                StatusCode::BAD_REQUEST,
                format!("Ticker with ID {id} not found."),
            ),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status_code, body).into_response()
    }
}

impl Ticker {
    pub fn new(id: i32, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            actions: 0,
        }
    }

    /// Regresa el precio en el que te sale comprar cierta cantidad de acciones según el estado actual del ticker.
    /// El precio se calcula con la siguiente formula: `precio = precio_base + (acciones_actuales * tasa_de_incremento)`.
    /// Cuando compras más de una acción de golpe, el precio no es el mismo para todas las acciones. Se debe calcular
    /// de forma individual para cada acción comprada. La suma de los precios individuales será el precio de la transacción.
    pub fn price_for_amount(&self, amount: i32) -> i32 {
        // Esta formula fue hecha por Jorgitox
        let current_price = BASE_PRICE + self.actions * INCREASE_RATE;
        current_price * amount + amount * (amount - 1) / 2 * INCREASE_RATE
    }
}

impl User {
    fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
            nicho_coins: 0,
        }
    }
}

/// Esta estructura representa el mercado. Todos los tickers están guardados aquí.
pub struct Market {
    tickers: Vec<Ticker>,
    users: Vec<User>,

    next_ticker_id: i32,
    next_user_id: i32,
}

impl Market {
    pub fn new() -> Self {
        Self {
            tickers: Vec::new(),
            users: Vec::new(),
            next_ticker_id: 0,
            next_user_id: 0,
        }
    }

    pub fn list_users(&self) -> Vec<User> {
        self.users.clone()
    }

    pub fn set_money_for_user(&mut self, user_id: i32, money: i32) -> Result<(), ApiError> {
        let user = self.get_user_by_id(user_id)?;
        user.nicho_coins = money;
        Ok(())
    }

    pub fn list_tickers(&self) -> Vec<Ticker> {
        self.tickers.clone()
    }

    pub fn add_ticker(&mut self, name: &str, description: &str) -> Ticker {
        let ticker = Ticker::new(
            self.next_ticker_id,
            name.to_string(),
            description.to_string(),
        );
        println!("Ticker '{name}' created (ID: {})", self.next_ticker_id);
        self.next_ticker_id += 1;
        self.tickers.push(ticker.clone());
        ticker
    }

    pub fn add_user(&mut self, name: &str) -> User {
        let user = User::new(self.next_user_id, name.to_string());
        println!("User '{name}' created (ID: {})", self.next_user_id);
        self.next_user_id += 1;
        self.users.push(user.clone());
        user
    }

    fn get_user_by_id(&mut self, user_id: i32) -> Result<&mut User, ApiError> {
        Ok(self
            .users
            .iter_mut()
            .find(|u| u.id == user_id)
            .ok_or(ApiError::UserNotFound(user_id))?)
    }

    // TODO: Manejo de errores propios en lugar de regresar Strings
    pub fn buy_actions(
        &mut self,
        user_id: i32,
        ticker_id: i32,
        amount: i32,
    ) -> Result<(), ApiError> {
        let user = self
            .users
            .iter_mut()
            .find(|u| u.id == user_id)
            .ok_or(ApiError::UserNotFound(user_id))?;
        let ticker = self
            .tickers
            .iter_mut()
            .find(|t| t.id == ticker_id)
            .ok_or(ApiError::TickerNotFound(ticker_id))?;

        let price = ticker.price_for_amount(amount);

        if user.nicho_coins < price {
            return Err(ApiError::BuyError);
        }

        ticker.actions += amount;
        user.nicho_coins -= price;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_buy_price_for_new_ticker() {
        let ticker = Ticker::new(0, "$JOGE".to_string(), "Dummy niche.".to_string());
        let result = ticker.price_for_amount(5);

        assert_eq!(result, 600);
    }

    #[test]
    fn correct_buy_price_for_ticker() {
        let mut ticker = Ticker::new(0, "$JOGE".to_string(), "Dummy niche.".to_string());
        ticker.actions = 5;
        let result = ticker.price_for_amount(5);

        assert_eq!(result, 850);
    }

    #[test]
    fn correct_buy_price_when_buying_one() {
        let ticker = Ticker::new(0, "$JOGE".to_string(), "Dummy niche.".to_string());
        let result = ticker.price_for_amount(1);

        assert_eq!(result, 100);
    }

    #[test]
    fn user_can_buy_actions_if_affordable() {
        let mut user = User::new(0, "Polarín".to_string());
        let mut ticker = Ticker::new(0, "$JOGE".to_string(), "Dummy niche.".to_string());
        user.nicho_coins = 17250;

        assert!(buy_actions(&mut user, &mut ticker, 50).is_ok());
    }

    #[test]
    fn user_cant_buy_actions_if_broke() {
        let mut user = User::new(0, "Polarín".to_string());
        let mut ticker = Ticker::new(0, "$JOGE".to_string(), "Dummy niche.".to_string());
        user.nicho_coins = 100;

        assert!(buy_actions(&mut user, &mut ticker, 50).is_err());
    }
}
