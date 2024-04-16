use crate::errors::Error;
use crate::proxies::{Proxiable, Proxy};
use crate::response::TurnstileResponse;
use crate::tasks::{merge, Task};

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Default)]
pub struct TurnstileTask {
    #[serde(skip_serializing)]
    pub id: Option<i64>,
    #[serde(rename = "type")]
    pub task_type: String,
    #[serde(rename = "websiteURL")]
    pub website_url: String,
    #[serde(rename = "websiteKey")]
    pub website_key: String,
    #[serde(skip_serializing)]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "turnstileCData")]
    pub turnstile_c_data: Option<String>,
    #[serde(skip_serializing)]
    pub proxy: Option<Proxy>,
}

impl TurnstileTask {
    pub fn new<I>(url: I, site_key: I) -> Self
    where
        I: Into<String>,
    {
        TurnstileTask {
            id: None,
            task_type: String::from("TurnstileTaskProxyless"),
            website_url: url.into(),
            website_key: site_key.into(),
            ..Default::default()
        }
    }

    pub fn set_action(mut self, payload: impl Into<String>) -> Self {
        self.action = Some(payload.into());
        self
    }

    pub fn set_turnstile_c_data(mut self, data: impl Into<String>) -> Self {
        self.turnstile_c_data = Some(data.into());
        self
    }
}

impl Task for TurnstileTask {
    type TaskResult = TurnstileResponse;

    fn get_task_id(&self) -> Option<i64> {
        self.id
    }

    fn set_task_id(&mut self, task_id: i64) {
        self.id = Some(task_id)
    }

    fn description(&self) -> String {
        String::from("Bypass Turnstile captcha")
    }

    fn as_value(&self) -> Result<Value, Error> {
        if let Some(proxy) = &self.proxy {
            let proxy_value = serde_json::to_value(proxy)?;
            let mut task_value = serde_json::to_value(self)?;
            merge(&mut task_value, proxy_value);
            return Ok(task_value);
        }
        Ok(serde_json::to_value(self)?)
    }
}

impl Proxiable for TurnstileTask {
    fn set_proxy(&mut self, proxy: Proxy) {
        self.proxy = Some(proxy);
        self.task_type = String::from("TurnstileTask");
    }
}
