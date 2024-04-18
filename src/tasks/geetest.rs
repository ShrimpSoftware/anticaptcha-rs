use crate::errors::Error;
use crate::proxies::{Proxiable, Proxy};
use crate::response::GeeTestResponse;
use crate::tasks::{merge, Task};

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeeTestTask {
    #[serde(skip_serializing)]
    pub id: Option<i64>,
    #[serde(rename = "type")]
    pub task_type: String,
    #[serde(rename = "websiteURL")]
    pub website_url: String,
    #[serde(rename = "gt")]
    pub website_gt: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "challenge")]
    pub challenge: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "geetestApiServerSubdomain"
    )]
    pub subdomain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "geetestGetLib")]
    pub getlib: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i8>,
    #[serde(skip_serializing)]
    pub proxy: Option<Proxy>,
}

impl GeeTestTask {
    pub fn new(url: impl Into<String>, public_gt_key: impl Into<String>) -> Self {
        GeeTestTask {
            id: None,
            task_type: String::from("GeeTestTaskProxyless"),
            website_url: url.into(),
            website_gt: public_gt_key.into(),
            ..Default::default()
        }
    }

    pub fn set_challenge(mut self, challenge: impl Into<String>) -> Self {
        self.challenge = Some(challenge.into());
        self
    }

    pub fn set_subdomain(mut self, subdomain: impl Into<String>) -> Self {
        self.subdomain = Some(subdomain.into());
        self
    }

    pub fn set_getlib(mut self, getlib_data: impl Into<String>) -> Self {
        self.getlib = Some(getlib_data.into());
        self
    }

    pub fn set_version(mut self, version: i8) -> Self {
        self.version = Some(version);
        self
    }
}

impl Task for GeeTestTask {
    type TaskResult = GeeTestResponse;

    fn get_task_id(&self) -> Option<i64> {
        self.id
    }

    fn set_task_id(&mut self, task_id: i64) {
        self.id = Some(task_id)
    }

    fn description(&self) -> String {
        String::from("Solve GeeTest automatically")
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

impl Proxiable for GeeTestTask {
    fn set_proxy(&mut self, proxy: Proxy) {
        self.proxy = Some(proxy);
        self.task_type = String::from("GeeTestTask");
    }
}
