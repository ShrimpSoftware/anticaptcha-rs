use crate::tasks::Task;
use std::collections::HashMap;

pub struct FunCaptchaTask {
    pub id: Option<i64>,
    pub website_url: String,
    pub website_key: String,
    pub subdomain: Option<String>,
    pub data: Option<String>,
}

impl FunCaptchaTask {
    pub fn new(url: String, public_key: String) -> Self {
        FunCaptchaTask {
            id: None,
            website_url: url,
            website_key: public_key,
            subdomain: None,
            data: None,
        }
    }

    pub fn set_subdomain(&mut self, subdomain: String) {
        self.subdomain = Some(subdomain)
    }

    pub fn set_data(&mut self, data: String) {
        self.data = Some(data);
    }
}

impl Task for FunCaptchaTask {
    type TaskResult = crate::models::FunCaptchaResponse;

    fn get_task_id(&self) -> Option<i64> {
        self.id
    }

    fn set_task_id(&mut self, task_id: i64) {
        self.id = Some(task_id)
    }

    fn task_type(&self) -> String {
        String::from("FunCaptchaTaskProxyless")
    }

    fn description(&self) -> String {
        String::from("Solve Funcaptcha automatically")
    }

    fn into_map(&self) -> HashMap<&str, String> {
        let mut map = HashMap::from([
            ("type", self.task_type()),
            ("websiteURL", self.website_url.clone()),
            ("websitePublicKey", self.website_key.clone()),
        ]);

        if let Some(subdomain) = &self.subdomain {
            map.insert("funcaptchaApiJSSubdomain", subdomain.clone());
        }

        if let Some(data) = &self.data {
            map.insert("data", data.clone());
        }

        map
    }
}
