use crate::error::{Result, ShilpError};
use reqwest::{Client as HttpClient, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

/// The main client for interacting with the Shilp API
pub struct Client {
    pub(crate) base_url: String,
    pub(crate) http_client: HttpClient,
    pub(crate) auth_token: Option<String>,
}

impl Client {
    /// Creates a new Shilp API client with default settings
    pub fn new(base_url: impl Into<String>) -> Self {
        let http_client = HttpClient::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http_client,
            auth_token: None,
        }
    }

    /// Creates a new Shilp API client with a custom HTTP client
    pub fn with_http_client(base_url: impl Into<String>, http_client: HttpClient) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http_client,
            auth_token: None,
        }
    }

    /// Creates a new Shilp API client with authentication token
    pub fn with_auth(base_url: impl Into<String>, auth_token: impl Into<String>) -> Self {
        let mut client = Self::new(base_url);
        client.auth_token = Some(auth_token.into());
        client
    }

    /// Performs an HTTP request and returns the deserialized response
    pub(crate) async fn do_request<T, B>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&B>,
        query_params: Option<&HashMap<String, String>>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.http_client.request(method, &url);

        if let Some(params) = query_params {
            request = request.query(params);
        }

        if let Some(body) = body {
            request = request.json(body);
        }

        if let Some(token) = &self.auth_token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;

        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            return Err(ShilpError::ApiError { message, status });
        }

        // Deserialize the response body to a json map
        let response_text = response.text().await?;
        log::debug!("Response Text: {}", response_text);
        let result = serde_json::from_str::<T>(&response_text)?;
        Ok(result)

        // let result = response.json::<T>().await?;
        // Ok(result)
    }

    /// Performs a file upload request
    pub(crate) async fn do_file_request(
        &self,
        method: reqwest::Method,
        path: &str,
        file_path: &std::path::Path,
    ) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);

        let file = tokio::fs::File::open(file_path).await?;
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file");

        let stream = tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new());
        let file_body = reqwest::Body::wrap_stream(stream);

        let form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::stream(file_body).file_name(file_name.to_string()),
        );

        let response = self.http_client.request(method, &url).multipart(form);
        let response = if let Some(token) = &self.auth_token {
            response.bearer_auth(token).send().await?
        } else {
            response.send().await?
        };

        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            return Err(ShilpError::ApiError { message, status });
        }

        Ok(())
    }

    /// Performs a request that returns a file response
    pub(crate) async fn do_request_with_file_response(
        &self,
        method: reqwest::Method,
        path: &str,
        query_params: Option<&HashMap<String, String>>,
    ) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.http_client.request(method, &url);

        if let Some(params) = query_params {
            request = request.query(params);
        }

        if let Some(token) = &self.auth_token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;

        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            return Err(ShilpError::ApiError { message, status });
        }

        Ok(response)
    }
}
