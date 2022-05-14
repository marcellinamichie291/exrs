#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_endpoint: String,
    pub recv_window: u64,
}

impl Config {
    /// Configure binance with default production endpoints
    /// # Examples
    /// ```
    /// use binance::config::Config;
    /// let config = Config::default();
    /// ```
    pub fn default() -> Config {
        Config {
            rest_api_endpoint: "https://api.binance.com".into(),
            ws_endpoint: "wss://stream.binance.com:9443".into(),
            recv_window: 5000,
        }
    }

    /// Configure binance with all testnet endpoints
    /// # Examples
    /// ```
    /// use binance::config::Config;
    /// let config = Config::testnet();
    /// ```
    pub fn testnet() -> Config {
        Config::default()
            .set_rest_api_endpoint("https://testnet.binance.vision")
            .set_ws_endpoint("wss://testnet.binance.vision")
    }

    pub fn set_rest_api_endpoint<T: Into<String>>(mut self, rest_api_endpoint: T) -> Self {
        self.rest_api_endpoint = rest_api_endpoint.into();
        self
    }

    pub fn set_ws_endpoint<T: Into<String>>(mut self, ws_endpoint: T) -> Self {
        self.ws_endpoint = ws_endpoint.into();
        self
    }

    pub fn set_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }
}
