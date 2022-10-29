use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "errorId")]
    pub error_id: i8,
    #[serde(rename = "errorCode")]
    pub error_code: String,
    #[serde(rename = "errorDescription")]
    pub error_description: String,
}

#[derive(Debug, Deserialize)]
pub struct TaskStatus<T> {
    pub status: String,
    pub solution: Option<T>,
    pub cost: Option<String>,
    pub ip: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: Option<i64>,
    #[serde(rename = "endTime")]
    pub end_time: Option<i64>,
    #[serde(rename = "solveCount")]
    pub solve_count: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TaskResponse {
    #[serde(rename = "taskId")]
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
