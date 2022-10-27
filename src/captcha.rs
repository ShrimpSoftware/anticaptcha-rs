use crate::errors::Error;
use crate::models::{APIRequest, BalanceResponse, ErrorResponse, TaskResponse, TaskStatus};
use crate::tasks::Task;

use serde::de::DeserializeOwned;

pub struct Anticaptcha {
    pub client: reqwest::Client,
    pub key: String,
}

impl Anticaptcha {
    pub fn new(key: String) -> Self {
        let client = reqwest::Client::new();
        Anticaptcha {
            client: client,
            key: key,
        }
    }

    pub async fn balance(&self) -> Result<BalanceResponse, Error> {
        let data = APIRequest {
            key: self.key.clone(),
            task: None,
            task_id: None,
        };

        let response: BalanceResponse =
            request(&self.client, String::from("/getBalance"), &data).await?;
        Ok(response)
    }

    pub async fn create_task<T: Task>(&self, task: &mut T) -> Result<(), Error> {
        let data = APIRequest {
            key: self.key.clone(),
            task: Some(task.into_map()),
            task_id: None,
        };

        let response: TaskResponse =
            request(&self.client, String::from("/createTask"), &data).await?;

        task.set_task_id(response.task_id);
        Ok(())
    }

    pub async fn wait_for<T: Task>(&self, task: &T) -> Result<TaskStatus<T::TaskResult>, Error> {
        let data = APIRequest {
            key: self.key.clone(),
            task: None,
            task_id: task.get_task_id(),
        };

        let mut response: TaskStatus<T::TaskResult> =
            request(&self.client, String::from("/getTaskResult"), &data).await?;
        while !response.solution.is_some() {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            response = request(&self.client, String::from("/getTaskResult"), &data).await?;
        }

        Ok(response)
    }
}

pub async fn request<T: DeserializeOwned>(
    client: &reqwest::Client,
    path: String,
    data: &APIRequest<'_>,
) -> Result<T, Error> {
    let response = client
        .post(format!("https://api.anti-captcha.com{}", path))
        .json(data)
        .send()
        .await?
        .text()
        .await?;

    if response.contains("errorCode") {
        let result: ErrorResponse = serde_json::from_str(&response)?;
        return Err(Error::ApiError(result));
    }

    let result: T = serde_json::from_str(&response)?;
    Ok(result)
}
