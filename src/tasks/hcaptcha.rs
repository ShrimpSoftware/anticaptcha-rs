use crate::errors::Error;
use crate::proxies::{Proxiable, Proxy};
use crate::response::RecaptchaV2Response;
use crate::tasks::{merge, Task};

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Default)]
pub struct HCaptchaTask {
    #[serde(skip_serializing)]
    pub id: Option<i64>,
    #[serde(rename = "type")]
    pub task_type: String,
    #[serde(rename = "websiteURL")]
    pub website_url: String,
    #[serde(rename = "websiteKey")]
    pub website_key: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    #[serde(rename = "isInvisible")]
    pub is_invisible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "enterprisePayload")]
    pub enterprise_payload: Option<String>,
    #[serde(skip_serializing)]
    pub proxy: Option<Proxy>,
}

impl HCaptchaTask {
    pub fn new<I>(url: I, site_key: I, user_agent: I) -> Self
    where
        I: Into<String>,
    {
        HCaptchaTask {
            id: None,
            task_type: String::from("HCaptchaTaskProxyless"),
            website_url: url.into(),
            website_key: site_key.into(),
            user_agent: user_agent.into(),
            ..Default::default()
        }
    }

    pub fn set_enterprise_payload(mut self, payload: impl Into<String>) -> Self {
        self.enterprise_payload = Some(payload.into());
        self
    }

    pub fn set_invisible(mut self, is_invisible: bool) -> Self {
        self.is_invisible = Some(is_invisible);
        self
    }
}

impl Task for HCaptchaTask {
    type TaskResult = RecaptchaV2Response;

    fn get_task_id(&self) -> Option<i64> {
        self.id
    }

    fn set_task_id(&mut self, task_id: i64) {
        self.id = Some(task_id)
    }

    fn description(&self) -> String {
        String::from("Solve HCaptcha automatically")
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

impl Proxiable for HCaptchaTask {
    fn set_proxy(&mut self, proxy: Proxy) {
        self.proxy = Some(proxy);
        self.task_type = String::from("HCaptchaTask");
    }
}
