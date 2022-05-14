use super::client::*;
use super::errors::*;
use super::rest_model::*;
use super::util::*;
use serde_json::{from_str, Value};
use std::collections::BTreeMap;

static API_V3_DEPTH: &str = "/api/v3/depth";
static API_V3_TICKER_PRICE: &str = "/api/v3/ticker/price";
static API_V3_AVG_PRICE: &str = "/api/v3/avgPrice";
static API_V3_BOOK_TICKER: &str = "/api/v3/ticker/bookTicker";
static API_V3_24H_TICKER: &str = "/api/v3/ticker/24hr";
static API_V3_KLINES: &str = "/api/v3/klines";

#[derive(Clone)]
pub struct Market {
    pub client: Client,
    pub recv_window: u64,
}

// Market Data endpoints
impl Market {
    fn symbol_request<S>(&self, symbol: S) -> String
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        build_request(&parameters)
    }

    /// Order book (Default 100; max 5000)
    /// # Examples
    /// ```rust
    /// use binance::{api::*, market::*, config::*};
    /// let market: Market = Binance::new_with_env(&Config::default());
    /// let orderbook = tokio_test::block_on(market.get_depth("BTCUSDT".to_string()));
    /// assert!(orderbook.is_ok(), "{:?}", orderbook);
    /// ```
    pub async fn get_depth<S>(&self, symbol: S) -> Result<OrderBookPartial>
    where
        S: Into<String>,
    {
        let request = self.symbol_request(symbol);
        let data = self.client.get(API_V3_DEPTH, &request).await?;
        let order_book: OrderBookPartial = from_str(data.as_str())?;

        Ok(order_book)
    }

    /// Order book with a custom depth limit
    /// Supported limits are: 5, 10, 20, 50, 100, 500, 1000, 5000
    /// # Examples
    /// ```rust
    /// use binance::{api::*, market::*, config::*};
    /// let market: Market = Binance::new_with_env(&Config::default());
    /// let orderbook = tokio_test::block_on(market.get_custom_depth("BTCUSDT".to_string(), 50));
    /// assert!(orderbook.is_ok(), "{:?}", orderbook);
    /// let bids_len = orderbook.unwrap().bids.len();
    /// assert_eq!(bids_len, 50);
    /// ```
    pub async fn get_custom_depth<S>(&self, symbol: S, limit: u16) -> Result<OrderBookPartial>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("limit".into(), limit.to_string());

        let request = build_request(&parameters);
        let data = self.client.get(API_V3_DEPTH, &request).await?;
        let order_book: OrderBookPartial = from_str(data.as_str())?;

        Ok(order_book)
    }

    /// Latest price for ALL symbols.
    /// # Examples
    /// ```rust
    /// use binance::{api::*, market::*, config::*};
    /// let market: Market = Binance::new_with_env(&Config::default());
    /// let prices = tokio_test::block_on(market.get_all_prices());
    /// assert!(prices.is_ok(), "{:?}", prices);
    /// ```
    pub async fn get_all_prices(&self) -> Result<Prices> {
        let data = self.client.get(API_V3_TICKER_PRICE, "").await?;

        let prices: Prices = from_str(data.as_str())?;

        Ok(prices)
    }

    /// Latest price for ONE symbol.
    /// # Examples
    /// ```rust
    /// use binance::{api::*, market::*, config::*};
    /// let market: Market = Binance::new_with_env(&Config::default());
    /// let price = tokio_test::block_on(market.get_price("BTCUSDT"));
    /// assert!(price.is_ok(), "{:?}", price);
    /// ```
    pub async fn get_price<S>(&self, symbol: S) -> Result<SymbolPrice>
    where
        S: Into<String>,
    {
        let request = self.symbol_request(symbol);
        let data = self.client.get(API_V3_TICKER_PRICE, &request).await?;
        let symbol_price: SymbolPrice = from_str(data.as_str())?;

        Ok(symbol_price)
    }

    /// Average price for ONE symbol.
    /// # Examples
    /// ```rust
    /// use binance::{api::*, market::*, config::*};
    /// let market: Market = Binance::new_with_env(&Config::default());
    /// let avg_price = tokio_test::block_on(market.get_average_price("BTCUSDT"));
    /// assert!(avg_price.is_ok(), "{:?}", avg_price);
    /// ```
    pub async fn get_average_price<S>(&self, symbol: S) -> Result<AveragePrice>
    where
        S: Into<String>,
    {
        let request = self.symbol_request(symbol);
        let data = self.client.get(API_V3_AVG_PRICE, &request).await?;
        let average_price: AveragePrice = from_str(data.as_str())?;

        Ok(average_price)
    }

    /// Symbols order book ticker
    /// -> Best price/qty on the order book for ALL symbols.
    /// # Examples
    /// ```rust
    /// use binance::{api::*, market::*, config::*};
    /// let market: Market = Binance::new_with_env(&Config::default());
    /// let tickers = tokio_test::block_on(market.get_all_book_tickers());
    /// assert!(tickers.is_ok(), "{:?}", tickers);
    /// ```
    pub async fn get_all_book_tickers(&self) -> Result<BookTickers> {
        let data = self.client.get(API_V3_BOOK_TICKER, "").await?;

        let book_tickers: BookTickers = from_str(data.as_str())?;

        Ok(book_tickers)
    }

    /// -> Best price/qty on the order book for ONE symbol
    /// # Examples
    /// ```rust
    /// use binance::{api::*, market::*, config::*};
    /// let market: Market = Binance::new_with_env(&Config::default());
    /// let tickers = tokio_test::block_on(market.get_book_ticker("BTCUSDT"));
    /// assert!(tickers.is_ok(), "{:?}", tickers);
    /// ```
    pub async fn get_book_ticker<S>(&self, symbol: S) -> Result<Tickers>
    where
        S: Into<String>,
    {
        let request = self.symbol_request(symbol);
        let data = self.client.get(API_V3_BOOK_TICKER, &request).await?;
        let ticker: Tickers = from_str(data.as_str())?;

        Ok(ticker)
    }

    /// 24hr ticker price change statistics
    /// # Examples
    /// ```rust
    /// use binance::{api::*, market::*, config::*};
    /// let market: Market = Binance::new_with_env(&Config::default());
    /// let price_stats = tokio_test::block_on(market.get_24h_price_stats("BTCUSDT"));
    /// assert!(price_stats.is_ok(), "{:?}", price_stats);
    /// ```
    pub async fn get_24h_price_stats<S>(&self, symbol: S) -> Result<PriceStats>
    where
        S: Into<String>,
    {
        let request = self.symbol_request(symbol);
        let data = self.client.get(API_V3_24H_TICKER, &request).await?;

        let stats: PriceStats = from_str(data.as_str())?;

        Ok(stats)
    }

    /// Returns up to 'limit' klines for given symbol and interval ("1m", "5m", ...)
    /// https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#klinecandlestick-data
    /// # Examples
    /// ```rust
    /// use binance::{api::*, market::*, config::*};
    /// let market: Market = Binance::new_with_env(&Config::default());
    /// let klines = tokio_test::block_on(market.get_klines("BTCUSDT", "1m", None, None, None));
    /// assert!(klines.is_ok(), "{:?}", klines);
    /// ```
    pub async fn get_klines<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        interval: S2,
        limit: S3,
        start_time: S4,
        end_time: S5,
    ) -> Result<KlineSummaries>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("interval".into(), interval.into());

        // Add three optional parameters
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{}", st));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{}", et));
        }

        let request = build_request(&parameters);

        let data = self.client.get(API_V3_KLINES, &request).await?;
        let parsed_data: Vec<Vec<Value>> = from_str(data.as_str())?;

        let klines = KlineSummaries::AllKlineSummaries(
            parsed_data
                .iter()
                .map(|row| KlineSummary {
                    open_time: to_i64(&row[0]),
                    open: to_f64(&row[1]),
                    high: to_f64(&row[2]),
                    low: to_f64(&row[3]),
                    close: to_f64(&row[4]),
                    volume: to_f64(&row[5]),
                    close_time: to_i64(&row[6]),
                    quote_asset_volume: to_f64(&row[7]),
                    number_of_trades: to_i64(&row[8]),
                    taker_buy_base_asset_volume: to_f64(&row[9]),
                    taker_buy_quote_asset_volume: to_f64(&row[10]),
                })
                .collect(),
        );
        Ok(klines)
    }
}
