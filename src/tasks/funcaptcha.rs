use crate::errors::Error;
use crate::proxies::{Proxiable, Proxy};
use crate::response::FunCaptchaResponse;
use crate::tasks::{merge, Task};

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FunCaptchaTask {
    #[serde(skip_serializing)]
    pub id: Option<i64>,
    #[serde(rename = "type")]
    pub task_type: String,
    #[serde(rename = "websiteURL")]
    pub website_url: String,
    #[serde(rename = "websitePublicKey")]
    pub website_key: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "funcaptchaApiJSSubdomain"
    )]
    pub subdomain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing)]
    pub proxy: Option<Proxy>,
}

impl FunCaptchaTask {
    pub fn new(url: impl Into<String>, public_key: impl Into<String>) -> Self {
        FunCaptchaTask {
            id: None,
            task_type: String::from("FunCaptchaTaskProxyless"),
            website_url: url.into(),
            website_key: public_key.into(),
            ..Default::default()
        }
    }

    pub fn set_subdomain(mut self, subdomain: impl Into<String>) -> Self {
        self.subdomain = Some(subdomain.into());
        self
    }

    pub fn set_data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }
}

impl Task for FunCaptchaTask {
    type TaskResult = FunCaptchaResponse;

    fn get_task_id(&self) -> Option<i64> {
        self.id
    }

    fn set_task_id(&mut self, task_id: i64) {
        self.id = Some(task_id)
    }

    fn description(&self) -> String {
        String::from("Solve Funcaptcha automatically")
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

impl Proxiable for FunCaptchaTask {
    fn set_proxy(&mut self, proxy: Proxy) {
        self.proxy = Some(proxy);
        self.task_type = String::from("FunCaptchaTask");
    }
}
