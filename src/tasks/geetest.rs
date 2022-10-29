use crate::tasks::Task;
use std::collections::HashMap;

pub struct GeeTestTask {
    pub id: Option<i64>,
    pub website_url: String,
    pub website_gt: String,
    pub challenge: Option<String>,
    pub subdomain: Option<String>,
    pub getlib: Option<String>,
    pub version: Option<i8>,
}

impl GeeTestTask {
    pub fn new(url: String, public_gt_key: String) -> Self {
        GeeTestTask {
            id: None,
            website_url: url,
            website_gt: public_gt_key,
            challenge: None,
            subdomain: None,
            getlib: None,
            version: None,
        }
    }

    pub fn set_challenge(&mut self, challenge: String) {
        self.challenge = Some(challenge)
    }

    pub fn set_subdomain(&mut self, subdomain: String) {
        self.subdomain = Some(subdomain);
    }

    pub fn set_getlib(&mut self, getlib_data: String) {
        self.getlib = Some(getlib_data);
    }

    pub fn set_version(&mut self, version: i8) {
        self.version = Some(version);
    }
}

impl Task for GeeTestTask {
    type TaskResult = crate::models::GeeTestResponse;

    fn get_task_id(&self) -> Option<i64> {
        self.id
    }

    fn set_task_id(&mut self, task_id: i64) {
        self.id = Some(task_id)
    }

    fn task_type(&self) -> String {
        String::from("GeeTestTaskProxyless")
    }

    fn description(&self) -> String {
        String::from("Solve GeeTest automatically")
    }

    fn into_map(&self) -> HashMap<&str, String> {
        let mut map = HashMap::from([
            ("type", self.task_type()),
            ("websiteURL", self.website_url.clone()),
            ("gt", self.website_gt.clone()),
        ]);

        if let Some(challenge) = &self.challenge {
            map.insert("challenge", challenge.clone());
        }

        if let Some(subdomain) = &self.subdomain {
            map.insert("geetestApiServerSubdomain", subdomain.clone());
        }

        if let Some(getlib) = &self.getlib {
            map.insert("geetestGetLib", getlib.clone());
        }

        if let Some(version) = &self.version {
            map.insert("version", version.to_string());
        }

        map
    }
}
