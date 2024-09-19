use error_chain::bail;

use crate::binance::model::Empty;
use crate::binance::futures::model::{ExchangeInformation, ServerTime, Symbol};
use crate::binance::client::Client;
use crate::common::errors::Result;
use crate::binance::api::API;
use crate::binance::api::Futures;

#[derive(Clone)]
pub struct FuturesGeneral {
    pub client: Client,
}

impl FuturesGeneral {
    // Test connectivity
    pub fn ping(&self) -> Result<String> {
        self.client
            .get::<Empty>(API::Futures(Futures::Ping), None)?;
        Ok("pong".into())
    }

    // Check server time
    pub fn get_server_time(&self) -> Result<ServerTime> {
        self.client.get(API::Futures(Futures::Time), None)
    }

    // Obtain exchange information
    // - Current exchange trading rules and symbol information
    pub fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get(API::Futures(Futures::ExchangeInfo), None)
    }

    // Get Symbol information
    pub fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
    where
        S: Into<String>,
    {
        let upper_symbol = symbol.into().to_uppercase();
        match self.exchange_info() {
            Ok(info) => {
                for item in info.symbols {
                    if item.symbol == upper_symbol {
                        return Ok(item);
                    }
                }
                bail!("Symbol not found")
            }
            Err(e) => Err(e),
        }
    }

    // Get All Symbol information
    pub fn get_all_symbol_info(&self) -> Result<Vec<Symbol>>
    {
        match self.exchange_info() {
            Ok(info) => Ok(info.symbols),
            Err(e) => Err(e),
        }
    }
}
