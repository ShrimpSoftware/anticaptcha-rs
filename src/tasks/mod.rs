use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub mod recaptcha_v2;
pub use recaptcha_v2::RecaptchaV2Task;

pub trait Task {
    type TaskResult: DeserializeOwned;

    fn get_task_id(&self) -> Option<i64>;
    fn set_task_id(&mut self, task_id: i64);
    fn task_type(&self) -> String;
    fn description(&self) -> String;
    fn into_map(&self) -> HashMap<&str, String>;
}
