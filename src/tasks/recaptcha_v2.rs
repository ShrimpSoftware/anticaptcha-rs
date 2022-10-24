use std::collections::HashMap;

pub trait Task {
    fn task_type(&self) -> String;
    fn description(&self) -> String;
    fn into_map(&self) -> HashMap<&str, String>;
}

pub struct RecaptchaV2Task {
    pub website_url: String,
    pub website_key: String,
    pub is_invisible: bool,
    pub data_s_value: Option<String>,
}

impl RecaptchaV2Task {
    pub fn new(url: &str, site_key: &str) -> Self {
        RecaptchaV2Task {
            website_url: String::from(url),
            website_key: String::from(site_key),
            is_invisible: true,
            data_s_value: None,
        }
    }

    pub fn set_sdata(&mut self, sdata: &str) {
        self.data_s_value = Some(String::from(sdata))
    }

    pub fn set_invisible(&mut self, is_invisible: bool) {
        self.is_invisible = is_invisible;
    }
}

impl Task for RecaptchaV2Task {
    fn task_type(&self) -> String {
        String::from("RecaptchaV2Task")
    }

    fn description(&self) -> String {
        String::from("Solve Google Recaptcha automatically")
    }

    fn into_map(&self) -> HashMap<&str, String> {
        HashMap::from([
            ("type", self.task_type()),
            ("websiteURL", self.website_url.clone()),
            ("websiteKey", self.website_key.clone()),
        ])
    }
}
