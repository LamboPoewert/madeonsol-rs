use serde::{de::DeserializeOwned, Serialize};

use crate::error::{MadeOnSolError, Result};

const BASE_URL: &str = "https://madeonsol.com/api/v1";

#[derive(Debug)]
pub(crate) struct HttpCore {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl HttpCore {
    pub(crate) fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("madeonsol-rs/0.17.0")
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self {
            client,
            base_url: BASE_URL.to_string(),
            api_key,
        }
    }

    pub(crate) async fn get<T, Q>(&self, path: &str, query: &Q) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .header("Accept", "application/json")
            .query(query)
            .send()
            .await?;
        Self::handle(resp).await
    }

    pub(crate) async fn post_empty<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await?;
        Self::handle(resp).await
    }

    pub(crate) async fn post_json<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .header("Accept", "application/json")
            .json(body)
            .send()
            .await?;
        Self::handle(resp).await
    }

    pub(crate) async fn patch_json<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .client
            .patch(&url)
            .bearer_auth(&self.api_key)
            .header("Accept", "application/json")
            .json(body)
            .send()
            .await?;
        Self::handle(resp).await
    }

    pub(crate) async fn delete<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .client
            .delete(&url)
            .bearer_auth(&self.api_key)
            .header("Accept", "application/json")
            .send()
            .await?;
        Self::handle(resp).await
    }

    async fn handle<T: DeserializeOwned>(resp: reqwest::Response) -> Result<T> {
        let status = resp.status();
        let body_text = resp.text().await?;

        if !status.is_success() {
            let body_json: serde_json::Value =
                serde_json::from_str(&body_text).unwrap_or_else(|_| {
                    serde_json::Value::String(body_text.clone())
                });
            let message = body_json
                .get("error")
                .or_else(|| body_json.get("message"))
                .and_then(|m| m.as_str())
                .map(String::from)
                .unwrap_or_else(|| format!("Request failed with status {}", status));
            return Err(MadeOnSolError::Api {
                status: status.as_u16(),
                message,
                body: body_json,
            });
        }

        serde_json::from_str(&body_text).map_err(MadeOnSolError::Json)
    }
}
