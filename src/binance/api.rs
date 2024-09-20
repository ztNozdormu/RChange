use crate::binance::account::Account;
use crate::binance::client::Client;
use crate::common::config::Config;
use crate::binance::futures::account::FuturesAccount;
use crate::binance::futures::general::FuturesGeneral;
use crate::binance::futures::market::FuturesMarket;
use crate::binance::futures::userstream::FuturesUserStream;
use crate::binance::general::General;
use crate::binance::market::Market;
use crate::binance::userstream::UserStream;
use crate::binance::savings::Savings;

#[allow(clippy::all)]
pub enum API {
    Spot(Spot),
    Savings(Sapi),
    Futures(Futures),
}

/// Endpoint for production and test orders.
///
/// Orders issued to test are validated, but not sent into the matching engine.
pub enum Spot {
    Ping,
    Time,
    ExchangeInfo,
    Depth,
    Trades,
    HistoricalTrades,
    AggTrades,
    Klines,
    AvgPrice,
    Ticker24hr,
    Price,
    BookTicker,
    Order,
    OrderTest,
    OpenOrders,
    AllOrders,
    Oco,
    OrderList,
    AllOrderList,
    OpenOrderList,
    Account,
    MyTrades,
    UserDataStream,
}

pub enum Sapi {
    AllCoins,
    AssetDetail,
    DepositAddress,
    SpotFuturesTransfer,
}

pub enum Futures {
    Ping,
    Time,
    ExchangeInfo,
    Depth,
    Trades,
    HistoricalTrades,
    AggTrades,
    Klines,
    ContinuousKlines,
    IndexPriceKlines,
    MarkPriceKlines,
    PremiumIndex,
    FundingRate,
    Ticker24hr,
    TickerPrice,
    BookTicker,
    AllForceOrders,
    AllOpenOrders,
    AllOrders,
    UserTrades,
    Order,
    PositionRisk,
    Balance,
    PositionSide,
    OpenInterest,
    OpenInterestHist,
    TopLongShortAccountRatio,
    TopLongShortPositionRatio,
    GlobalLongShortAccountRatio,
    TakerlongshortRatio,
    LvtKlines,
    IndexInfo,
    ChangeInitialLeverage,
    MarginType,
    PositionMargin,
    Account,
    OpenOrders,
    UserDataStream,
    Income,
}

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::Spot(route) => match route {
                Spot::Ping => "/api/v3/ping",
                Spot::Time => "/api/v3/time",
                Spot::ExchangeInfo => "/api/v3/exchangeInfo",
                Spot::Depth => "/api/v3/depth",
                Spot::Trades => "/api/v3/trades",
                Spot::HistoricalTrades => "/api/v3/historicalTrades",
                Spot::AggTrades => "/api/v3/aggTrades",
                Spot::Klines => "/api/v3/klines",
                Spot::AvgPrice => "/api/v3/avgPrice",
                Spot::Ticker24hr => "/api/v3/ticker/24hr",
                Spot::Price => "/api/v3/ticker/price",
                Spot::BookTicker => "/api/v3/ticker/bookTicker",
                Spot::Order => "/api/v3/order",
                Spot::OrderTest => "/api/v3/order/test",
                Spot::OpenOrders => "/api/v3/openOrders",
                Spot::AllOrders => "/api/v3/allOrders",
                Spot::Oco => "/api/v3/order/oco",
                Spot::OrderList => "/api/v3/orderList",
                Spot::AllOrderList => "/api/v3/allOrderList",
                Spot::OpenOrderList => "/api/v3/openOrderList",
                Spot::Account => "/api/v3/account",
                Spot::MyTrades => "/api/v3/myTrades",
                Spot::UserDataStream => "/api/v3/userDataStream",
            },
            API::Savings(route) => match route {
                Sapi::AllCoins => "/sapi/v1/capital/config/getall",
                Sapi::AssetDetail => "/sapi/v1/asset/assetDetail",
                Sapi::DepositAddress => "/sapi/v1/capital/deposit/address",
                Sapi::SpotFuturesTransfer => "/sapi/v1/futures/transfer",
            },
            API::Futures(route) => match route {
                Futures::Ping => "/fapi/v1/ping",
                Futures::Time => "/fapi/v1/time",
                Futures::ExchangeInfo => "/fapi/v1/exchangeInfo",
                Futures::Depth => "/fapi/v1/depth",
                Futures::Trades => "/fapi/v1/trades",
                Futures::HistoricalTrades => "/fapi/v1/historicalTrades",
                Futures::AggTrades => "/fapi/v1/aggTrades",
                Futures::Klines => "/fapi/v1/klines",
                Futures::ContinuousKlines => "/fapi/v1/continuousKlines",
                Futures::IndexPriceKlines => "/fapi/v1/indexPriceKlines",
                Futures::MarkPriceKlines => "/fapi/v1/markPriceKlines",
                Futures::PremiumIndex => "/fapi/v1/premiumIndex",
                Futures::FundingRate => "/fapi/v1/fundingRate",
                Futures::Ticker24hr => "/fapi/v1/ticker/24hr",
                Futures::TickerPrice => "/fapi/v1/ticker/price",
                Futures::BookTicker => "/fapi/v1/ticker/bookTicker",
                Futures::AllForceOrders => "/fapi/v1/allForceOrders",
                Futures::AllOpenOrders => "/fapi/v1/allOpenOrders",
                Futures::AllOrders => "/fapi/v1/allOrders",
                Futures::UserTrades => "/fapi/v1/userTrades",
                Futures::PositionSide => "/fapi/v1/positionSide/dual",
                Futures::Order => "/fapi/v1/order",
                Futures::PositionRisk => "/fapi/v2/positionRisk",
                Futures::Balance => "/fapi/v2/balance",
                Futures::OpenInterest => "/fapi/v1/openInterest",
                Futures::OpenInterestHist => "/futures/data/openInterestHist",
                Futures::TopLongShortAccountRatio => "/futures/data/topLongShortAccountRatio",
                Futures::TopLongShortPositionRatio => "/futures/data/topLongShortPositionRatio",
                Futures::GlobalLongShortAccountRatio => "/futures/data/globalLongShortAccountRatio",
                Futures::TakerlongshortRatio => "/futures/data/takerlongshortRatio",
                Futures::LvtKlines => "/fapi/v1/lvtKlines",
                Futures::IndexInfo => "/fapi/v1/indexInfo",
                Futures::ChangeInitialLeverage => "/fapi/v1/leverage",
                Futures::MarginType => "/fapi/v1/marginType",
                Futures::PositionMargin => "/fapi/v1/positionMargin",
                Futures::Account => "/fapi/v2/account",
                Futures::OpenOrders => "/fapi/v1/openOrders",
                Futures::UserDataStream => "/fapi/v1/listenKey",
                Futures::Income => "/fapi/v1/income",
            },
        })
    }
}

pub trait Binance {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self;
    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Self;
}

impl Binance for General {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> General {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> General {
        General {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
        }
    }
}

impl Binance for Account {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Account {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Account {
        Account {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for Savings {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for Market {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Market {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Market {
        Market {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for UserStream {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> UserStream {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> UserStream {
        UserStream {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

// *****************************************************
//              Binance Futures API
// *****************************************************

impl Binance for FuturesGeneral {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesGeneral {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesGeneral {
        FuturesGeneral {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
        }
    }
}

impl Binance for FuturesMarket {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesMarket {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesMarket {
        FuturesMarket {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for FuturesAccount {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Self {
        Self {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for FuturesUserStream {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesUserStream {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesUserStream {
        FuturesUserStream {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
            recv_window: config.recv_window,
        }
    }
}

/// A `enum` that represents the kline period of the Binance kline period.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[warn(non_camel_case_types)]
pub enum BinanceInterval {
    m1,
    m3,
    m5,
    m15,
    m30,

    h1,
    h2,
    h4,
    h6,
    h8,
    h12,

    d1,
    d3,

    w1,

    M1,
}

impl BinanceInterval {
    pub fn name(self) -> String {
        // 周期字符串名称
        match self {
            BinanceInterval::m1 => "1m".to_string(),
            BinanceInterval::m3 => "3m".to_string(),
            BinanceInterval::m5 => "5m".to_string(),
            BinanceInterval::m15 => "15m".to_string(),
            BinanceInterval::m30 => "30m".to_string(),

            BinanceInterval::h1 => "1h".to_string(),
            BinanceInterval::h2 => "2h".to_string(),
            BinanceInterval::h4 => "4h".to_string(),
            BinanceInterval::h6 => "6h".to_string(),
            BinanceInterval::h8 => "8h".to_string(),
            BinanceInterval::h12 => "12h".to_string(),

            BinanceInterval::d1 => "1d".to_string(),
            BinanceInterval::d3 => "3d".to_string(),

            BinanceInterval::w1 => "1w".to_string(),

            BinanceInterval::M1 => "1M".to_string(),
        }
    }
    // 周期对应的毫秒值
    pub fn value(self) -> u64 {
        match self {
            BinanceInterval::m1 => Duration::from_secs(1 * 60).as_secs(),
            BinanceInterval::m3 => Duration::from_secs(3 * 60).as_secs(),
            BinanceInterval::m5 => Duration::from_secs(5 * 60).as_secs(),
            BinanceInterval::m15 => Duration::from_secs(15 * 60).as_secs(),
            BinanceInterval::m30 => Duration::from_secs(30 * 60).as_secs(),

            BinanceInterval::h1 => Duration::from_secs(1 * 3600).as_secs(),
            BinanceInterval::h2 => Duration::from_secs(2 * 3600).as_secs(),
            BinanceInterval::h4 => Duration::from_secs(4 * 3600).as_secs(),
            BinanceInterval::h6 => Duration::from_secs(6 * 3600).as_secs(),
            BinanceInterval::h8 => Duration::from_secs(8 * 3600).as_secs(),
            BinanceInterval::h12 => Duration::from_secs(12 * 3600).as_secs(),

            BinanceInterval::d1 => Duration::from_secs(1 * 24 * 3600).as_secs(),
            BinanceInterval::d3 => Duration::from_secs(3 * 24 * 3600).as_secs(),
            BinanceInterval::w1 => Duration::from_secs(7 * 24 * 3600).as_secs(),
            BinanceInterval::M1 => {
                Duration::from_secs(get_current_month_days() * 24 * 3600).as_secs()
            }
        }
    }
    // 周期枚举循环迭代函数 暂时只取15m 30m 2h 4h 8h d1 数据
    pub fn iterator() -> impl Iterator<Item = BinanceInterval> {
        [
            BinanceInterval::m5,
            BinanceInterval::m15,
            BinanceInterval::m30,
            BinanceInterval::h2,
            BinanceInterval::h4,
            BinanceInterval::h8,
            BinanceInterval::d1,
        ]
        .into_iter()
    }
}
/**
 * 获取当前月天数
 */
fn get_current_month_days() -> u64 {
    // 获取当前日期
    let today = Utc::now().naive_utc().date();

    let year = today.year();
    let month = today.month();

    // 获取下个月的第1天
    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    };

    // 当前月的第1天
    let this_month = NaiveDate::from_ymd_opt(year, month, 1);

    // 计算当前月的天数
    let days_in_month = (next_month.unwrap() - this_month.unwrap()).num_days() as u64;

    days_in_month
}
