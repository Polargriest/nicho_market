use crate::{
    logic::errors::ApiError,
    schema::{Ticker, User},
};

/// Esta estructura representa el mercado. Todos los tickers y los usuarios están guardados aquí.
/// Las transacciones de todo el mercado también se hacen aquí.
pub struct Market {
    tickers: Vec<Ticker>,
    users: Vec<User>,

    next_ticker_id: i32,
    next_user_id: i32,
}

impl Market {
    /// Crea un nuevo mercado. Solo debería haber un mercado al mismo tiempo, pues representa el servidor.
    /// Por ahora, solo regresa un mercado nuevo. Luego, esto se conectará con una base de datos para
    /// que sus campos representen el mercado real.
    pub fn new() -> Self {
        Self {
            tickers: Vec::new(),
            users: Vec::new(),
            next_ticker_id: 0,
            next_user_id: 0,
        }
    }

    //// USUARIOS ////

    /// Regresa una lista de todos los usuarios en un vector. Nota que se crea un clon de la lista real,
    /// por lo que se espera que este método no debe usarse si se quiere modificar la lista.
    pub fn list_users(&self) -> Vec<User> {
        self.users.clone()
    }

    /// Regresa una referencia mutable a un usuario del mercado. La referencia que se regresa apunta
    /// directamente al vector del mercado, por lo que se espera que se use esta función cuando necesites
    /// modificar los datos de un usuario.
    fn get_user_by_id(&mut self, user_id: i32) -> Result<&mut User, ApiError> {
        Ok(self
            .users
            .iter_mut()
            .find(|u| u.id == user_id)
            .ok_or(ApiError::UserNotFound(user_id))?)
    }

    /// Crea a un nuevo usuario en el mercado y lo mete a la lista. El ID del usuario se autogenera.
    /// No se llenan los huecos vacíos, sino que el ID siempre incrementa en uno.
    pub fn add_user(&mut self, name: &str) -> User {
        let user = User::new(self.next_user_id, name.to_string());
        println!("(+) User '{name}' created (ID: {})", self.next_user_id);
        self.next_user_id += 1;
        self.users.push(user.clone());
        user
    }

    //// TICKERS ////

    /// Regresa una lista de todos los tickers (o nichos) en un vector. Nota que se crea un clon de la
    /// lista real, por lo que se espera que este método no debe usarse si se quiere modificar la lista.
    pub fn list_tickers(&self) -> Vec<Ticker> {
        self.tickers.clone()
    }

    /// Crea un nuevo ticker (o nicho) en el mercado y lo mete a la lista. El ID del ticker se autogenera.
    /// No se llenan los huecos vacíos, sino que el ID siempre incrementa en uno.
    pub fn add_ticker(&mut self, name: &str, description: &str) -> Ticker {
        let ticker = Ticker::new(
            self.next_ticker_id,
            name.to_string(),
            description.to_string(),
        );
        println!("(+) Ticker '{name}' created (ID: {})", self.next_ticker_id);
        self.next_ticker_id += 1;
        self.tickers.push(ticker.clone());
        ticker
    }

    pub fn set_money_for_user(&mut self, user_id: i32, money: i32) -> Result<(), ApiError> {
        let user = self.get_user_by_id(user_id)?;
        user.nicho_coins = money;
        Ok(())
    }

    //// NEGOCIO ////

    /// Método que representa la compra de acciones que un usuario hace. Este método se encarga
    /// de descontarle el dinero al usuario (haciendo las validaciones pertinentes) y aumentar el
    /// número de acciones de un ticker.
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
            return Err(BuyError::NotAffordable);
        }

        ticker.actions += amount;
        user.nicho_coins -= price;

        Ok(())
    }
}
