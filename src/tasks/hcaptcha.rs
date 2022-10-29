use crate::tasks::Task;
use std::collections::HashMap;

pub struct HCaptchaTask {
    pub id: Option<i64>,
    pub website_url: String,
    pub website_key: String,
    pub user_agent: String,
    pub is_invisible: Option<bool>,
    pub enterprise_payload: Option<String>,
}

impl HCaptchaTask {
    pub fn new(url: String, site_key: String, user_agent: String) -> Self {
        HCaptchaTask {
            id: None,
            website_url: url,
            website_key: site_key,
            user_agent: user_agent,
            is_invisible: None,
            enterprise_payload: None,
        }
    }

    pub fn set_enterprise_payload(&mut self, payload: String) {
        self.enterprise_payload = Some(payload)
    }

    pub fn set_invisible(&mut self, is_invisible: bool) {
        self.is_invisible = Some(is_invisible);
    }
}

impl Task for HCaptchaTask {
    type TaskResult = crate::models::RecaptchaV2Response;

    fn get_task_id(&self) -> Option<i64> {
        self.id
    }

    fn set_task_id(&mut self, task_id: i64) {
        self.id = Some(task_id)
    }

    fn task_type(&self) -> String {
        String::from("HCaptchaTaskProxyless")
    }

    fn description(&self) -> String {
        String::from("Solve HCaptcha automatically")
    }

    fn into_map(&self) -> HashMap<&str, String> {
        let mut map = HashMap::from([
            ("type", self.task_type()),
            ("websiteURL", self.website_url.clone()),
            ("websiteKey", self.website_key.clone()),
        ]);

        if let Some(data_value) = &self.enterprise_payload {
            map.insert("enterprisePayload", data_value.clone());
        }

        if let Some(visibility) = &self.is_invisible {
            map.insert("isInvisible", visibility.to_string());
        }

        map
    }
}
