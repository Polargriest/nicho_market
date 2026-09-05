use std::collections::HashMap;

use crate::{
    logic::errors::ApiError,
    schema::{Ticker, User},
};

const INCREASE_RATE: i32 = 10;
pub const BASE_PRICE: i32 = 100;

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
    pub fn price_for_buying(&self, amount: i32) -> i32 {
        // Esta formula fue hecha por Jorgitox
        let current_price = BASE_PRICE + self.actions * INCREASE_RATE;
        current_price * amount + amount * (amount - 1) / 2 * INCREASE_RATE
    }

    pub fn price_for_selling(&self, amount: i32) -> Result<i32, ApiError> {
        if amount > self.actions {
            return Err(ApiError::NotEnoughActions);
        }

        let current_price = BASE_PRICE + self.actions * INCREASE_RATE;
        Ok(current_price * amount - amount * (amount + 1) / 2 * INCREASE_RATE)
    }
}

impl User {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
            nicho_coins: 0,
            portfolio: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_buy_price_for_new_ticker() {
        let ticker = Ticker::new(0, "$JOGE".to_string(), "Dummy niche.".to_string());
        let result = ticker.price_for_buying(5);

        assert_eq!(result, 600);
    }

    #[test]
    fn correct_buy_price_for_ticker() {
        let mut ticker = Ticker::new(0, "$JOGE".to_string(), "Dummy niche.".to_string());
        ticker.actions = 5;
        let result = ticker.price_for_buying(5);

        assert_eq!(result, 850);
    }

    #[test]
    fn correct_buy_price_when_buying_one() {
        let ticker = Ticker::new(0, "$JOGE".to_string(), "Dummy niche.".to_string());
        let result = ticker.price_for_buying(1);

        assert_eq!(result, 100);
    }

    #[test]
    fn correct_sell_price_for_ticker() {
        let mut ticker = Ticker::new(0, "$JOGE".to_string(), "Dummy niche.".to_string());
        ticker.actions = 10;
        let result = ticker.price_for_selling(10).unwrap();

        assert_eq!(result, 1450);
    }

    #[test]
    fn ticker_has_not_enough_actions_when_selling() {
        let ticker = Ticker::new(0, "$JOGE".to_string(), "Dummy niche.".to_string());
        let result = ticker.price_for_selling(5);

        assert!(result.is_err())
    }
}
