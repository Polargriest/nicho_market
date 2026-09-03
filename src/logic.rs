use crate::schema::{Ticker, User};

const INCREASE_RATE: i32 = 10;
pub const BASE_PRICE: i32 = 100;

impl Ticker {
    pub fn new(name: String, description: String) -> Self {
        Self {
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
    fn new(name: String) -> Self {
        Self {
            name,
            nicho_coins: 0,
        }
    }
}

/// Esta estructura representa el mercado. Todos los tickers están guardados aquí.
pub struct Market {
    tickers: Vec<Ticker>,
    users: Vec<User>,
}

impl Market {
    pub fn new() -> Self {
        Self {
            tickers: Vec::new(),
            users: Vec::new(),
        }
    }
}

// TODO: Manejo de errores propios en lugar de regresar Strings
pub fn buy_actions(mut user: User, mut ticker: Ticker, amount: i32) -> Result<(), String> {
    let price = ticker.price_for_amount(amount);

    if user.nicho_coins < price {
        return Err("User can't afford money".to_string());
    }

    ticker.actions += amount;
    user.nicho_coins -= price;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_buy_price_for_new_ticker() {
        let ticker = Ticker::new("$JOGE".to_string(), "Las jogeadas, muy buenas.".to_string());
        let result = ticker.price_for_amount(5);

        assert_eq!(result, 600);
    }

    #[test]
    fn correct_buy_price_for_ticker() {
        let mut ticker = Ticker::new("$JOGE".to_string(), "Las jogeadas, muy buenas.".to_string());
        ticker.actions = 5;
        let result = ticker.price_for_amount(5);

        assert_eq!(result, 850);
    }

    #[test]
    fn correct_buy_price_when_buying_one() {
        let ticker = Ticker::new("$JOGE".to_string(), "Las jogeadas, muy buenas.".to_string());
        let result = ticker.price_for_amount(1);

        assert_eq!(result, 100);
    }

    #[test]
    fn user_can_buy_actions_if_affordable() {
        let mut user = User::new("Polarín".to_string());
        let ticker = Ticker::new("$JOGE".to_string(), "Las jogeadas, muy buenas.".to_string());
        user.nicho_coins = 17250;

        assert!(buy_actions(user, ticker, 50).is_ok());
    }

    #[test]
    fn user_cant_buy_actions_if_broke() {
        let mut user = User::new("Polarín".to_string());
        let ticker = Ticker::new("$JOGE".to_string(), "Las jogeadas, muy buenas.".to_string());
        user.nicho_coins = 100;

        assert!(buy_actions(user, ticker, 50).is_err());
    }
}
