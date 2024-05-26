use crate::http::{request::Request, Credentials, Method};

/// `POST /sapi/v1/margin/max-leverage`
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
/// use rust_decimal_macros::dec;
///
/// let request = margin::max_leverage();
/// ```
pub struct MarginMaxLeverage {
    max_leverage: u64,
    credentials: Option<Credentials>,
}

impl MarginMaxLeverage {
    pub fn new(max_leverage: u64) -> Self {
        Self {
            max_leverage,
            credentials: None,
        }
    }

    pub fn max_leverage(mut self, max_leverage: u64) -> Self {
        self.max_leverage = max_leverage;
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<MarginMaxLeverage> for Request {
    fn from(request: MarginMaxLeverage) -> Request {
        let params = vec![("maxLeverage".to_owned(), request.max_leverage.to_string())];

        Request {
            path: "/sapi/v1/margin/max-leverage".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginMaxLeverage;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_max_leverage() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginMaxLeverage::new(10).credentials(&credentials).into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/max-leverage".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("maxLeverage".to_owned(), "10".to_string()),
                    ("amount".to_owned(), "1.01".to_string()),
                ],
                sign: true
            }
        );
    }
}
