use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error_id: i8,
    pub error_code: String,
    pub error_description: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStatus<T> {
    pub status: String,
    pub solution: Option<T>,
    pub cost: Option<String>,
    pub ip: Option<String>,
    pub create_time: Option<i64>,
    pub end_time: Option<i64>,
    pub solve_count: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResponse {
    pub task_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct BalanceResponse {
    pub balance: f64,
}

#[derive(Debug, Deserialize)]
pub struct RecaptchaV2Response {
    #[serde(rename = "gRecaptchaResponse")]
    pub grecaptcha: String,
}

#[derive(Debug, Deserialize)]
pub struct FunCaptchaResponse {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct GeeTestResponse {
    pub challenge: String,
    pub validate: String,
    pub seccode: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnstileResponse {
    pub token: String,
    pub user_agent: String,
}
