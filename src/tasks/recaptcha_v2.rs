use crate::tasks::Task;
use std::collections::HashMap;

pub struct RecaptchaV2Task {
    pub id: Option<i64>,
    pub website_url: String,
    pub website_key: String,
    pub is_invisible: Option<bool>,
    pub data_s_value: Option<String>,
}

impl RecaptchaV2Task {
    pub fn new(url: String, site_key: String) -> Self {
        RecaptchaV2Task {
            id: None,
            website_url: url,
            website_key: site_key,
            is_invisible: None,
            data_s_value: None,
        }
    }

    pub fn set_sdata(&mut self, sdata: String) {
        self.data_s_value = Some(sdata)
    }

    pub fn set_invisible(&mut self, is_invisible: bool) {
        self.is_invisible = Some(is_invisible);
    }
}

impl Task for RecaptchaV2Task {
    type TaskResult = crate::models::RecaptchaV2Response;

    fn get_task_id(&self) -> Option<i64> {
        self.id
    }

    fn set_task_id(&mut self, task_id: i64) {
        self.id = Some(task_id)
    }

    fn task_type(&self) -> String {
        String::from("RecaptchaV2TaskProxyless")
    }

    fn description(&self) -> String {
        String::from("Solve Google Recaptcha automatically (proxyless)")
    }

    fn into_map(&self) -> HashMap<&str, String> {
        let mut map = HashMap::from([
            ("type", self.task_type()),
            ("websiteURL", self.website_url.clone()),
            ("websiteKey", self.website_key.clone()),
        ]);

        if let Some(data_value) = &self.data_s_value {
            map.insert("recaptchaDataSValue", data_value.clone());
        }

        if let Some(visibility) = &self.is_invisible {
            map.insert("isInvisible", visibility.to_string());
        }

        map
    }
}
