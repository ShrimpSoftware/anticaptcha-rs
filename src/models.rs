use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub errorId: i8,
    pub errorCode: String,
    pub errorDescription: String,
}

#[derive(Debug, Deserialize)]
pub struct BalanceResponse {
    pub balance: f64,
}
