use crate::{
    logic::errors::ApiError,
    schema::{Ticker, User},
};

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
}
