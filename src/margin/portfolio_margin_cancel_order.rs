use crate::http::{request::Request, Credentials, Method};

/// `DELETE /papi/v1/margin/order`
///
/// Cancel an active order for margin account.
///
/// Either `orderId` or `origClientOrderId` must be sent.
///
/// Weight(IP): 2
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::portfolio_margin_cancel_order("BNBUSDT").order_id(10);
/// ```
pub struct PortfolioMarginCancelOrder {
    symbol: String,
    order_id: Option<u64>,
    orig_client_order_id: Option<String>,
    new_client_order_id: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl PortfolioMarginCancelOrder {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_id: None,
            orig_client_order_id: None,
            new_client_order_id: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn orig_client_order_id(mut self, orig_client_order_id: &str) -> Self {
        self.orig_client_order_id = Some(orig_client_order_id.to_owned());
        self
    }

    pub fn new_client_order_id(mut self, new_client_order_id: &str) -> Self {
        self.new_client_order_id = Some(new_client_order_id.to_owned());
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<PortfolioMarginCancelOrder> for Request {
    fn from(request: PortfolioMarginCancelOrder) -> Request {
        let mut params = vec![("symbol".to_owned(), request.symbol.to_string())];

        if request.order_id.is_none() && request.orig_client_order_id.is_none() {
            // todo return result
            println!("At least one order_id or orig_client_order_id must be present");
        }

        if let Some(order_id) = request.order_id {
            params.push(("orderId".to_owned(), order_id.to_string()));
        }

        if let Some(orig_client_order_id) = request.orig_client_order_id {
            params.push(("origClientOrderId".to_owned(), orig_client_order_id));
        }

        if let Some(new_client_order_id) = request.new_client_order_id {
            params.push(("newClientOrderId".to_owned(), new_client_order_id));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/papi/v1/margin/order".to_owned(),
            method: Method::Delete,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PortfolioMarginCancelOrder;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_cancel_order_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = PortfolioMarginCancelOrder::new("BNBUSDT")
            .order_id(10)
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/order".to_owned(),
                credentials: Some(credentials),
                method: Method::Delete,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("orderId".to_owned(), "10".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
