use crate::error::{Result, ShilpError};
use crate::models::{
    DiscoveryStats, GenericResponse, RegisterToDiscoveryRequest, ReplicaType, SyncStatus,
    UpdateSyncStatusRequest,
};
use reqwest::Client as HttpClient;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

/// The Discovery client for interacting with the Shilp Discovery API
pub struct DiscoveryClient {
    base_url: String,
    http_client: HttpClient,
}

impl DiscoveryClient {
    /// Creates a new Discovery API client with default settings
    pub fn new(base_url: impl Into<String>) -> Self {
        let http_client = HttpClient::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http_client,
        }
    }

    /// Creates a new Discovery API client with a custom HTTP client
    pub fn with_http_client(base_url: impl Into<String>, http_client: HttpClient) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http_client,
        }
    }

    /// Performs an HTTP request and returns the deserialized response
    async fn do_request<T, B>(
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

        let response = request.send().await?;

        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            return Err(ShilpError::ApiError { message, status });
        }

        let result = response.json::<T>().await?;
        Ok(result)
    }

    /// Gets Shilp statistics
    pub async fn get_shilp_stats(&self, account_id: &str) -> Result<DiscoveryStats> {
        let mut params = HashMap::new();
        params.insert("account_id".to_string(), account_id.to_string());

        self.do_request::<DiscoveryStats, ()>(
            reqwest::Method::GET,
            "/control/shilp/stats",
            None,
            Some(&params),
        )
        .await
    }

    /// Updates Shilp sync status
    pub async fn update_shilp_sync_status(
        &self,
        account_id: &str,
        address: &str,
        status: SyncStatus,
    ) -> Result<GenericResponse> {
        let req = UpdateSyncStatusRequest {
            account_id: account_id.to_string(),
            address: address.to_string(),
            status,
        };

        self.do_request(
            reqwest::Method::POST,
            "/control/shilp/sync-status",
            Some(&req),
            None,
        )
        .await
    }

    /// Registers a Shilp service
    pub async fn register_shilp_service(
        &self,
        account_id: &str,
        address: &str,
        id: &str,
        replica_type: ReplicaType,
    ) -> Result<()> {
        let mut registration_data = RegisterToDiscoveryRequest {
            account_id: account_id.to_string(),
            address: address.to_string(),
            id: id.to_string(),
            is_read: false,
            is_write: false,
        };

        match replica_type {
            ReplicaType::Read => {
                registration_data.is_read = true;
                self.register_shilp(&registration_data).await?;
            }
            ReplicaType::Write => {
                registration_data.is_write = true;
                self.register_shilp(&registration_data).await?;
            }
            ReplicaType::SingleNode => {
                registration_data.is_read = true;
                self.register_shilp(&registration_data).await?;

                registration_data.is_read = false;
                registration_data.is_write = true;
                self.register_shilp(&registration_data).await?;
            }
        }

        Ok(())
    }

    /// Unregisters a Shilp service
    pub async fn unregister_shilp_service(
        &self,
        account_id: &str,
        address: &str,
        id: &str,
        replica_type: ReplicaType,
    ) -> Result<()> {
        let mut registration_data = RegisterToDiscoveryRequest {
            account_id: account_id.to_string(),
            address: address.to_string(),
            id: id.to_string(),
            is_read: false,
            is_write: false,
        };

        match replica_type {
            ReplicaType::Read => {
                registration_data.is_read = true;
                self.unregister_shilp(&registration_data).await?;
            }
            ReplicaType::Write => {
                registration_data.is_write = true;
                self.unregister_shilp(&registration_data).await?;
            }
            ReplicaType::SingleNode => {
                registration_data.is_read = true;
                self.unregister_shilp(&registration_data).await?;

                registration_data.is_read = false;
                registration_data.is_write = true;
                self.unregister_shilp(&registration_data).await?;
            }
        }

        Ok(())
    }

    async fn register_shilp(
        &self,
        payload: &RegisterToDiscoveryRequest,
    ) -> Result<GenericResponse> {
        self.do_request(
            reqwest::Method::POST,
            "/control/shilp/register",
            Some(payload),
            None,
        )
        .await
    }

    async fn unregister_shilp(
        &self,
        payload: &RegisterToDiscoveryRequest,
    ) -> Result<GenericResponse> {
        self.do_request(
            reqwest::Method::POST,
            "/control/shilp/unregister",
            Some(payload),
            None,
        )
        .await
    }

    /// Registers a TEI service
    pub async fn register_tei_service(
        &self,
        account_id: &str,
        address: &str,
        id: &str,
    ) -> Result<()> {
        let registration_data = RegisterToDiscoveryRequest {
            account_id: account_id.to_string(),
            address: address.to_string(),
            id: id.to_string(),
            is_read: true,
            is_write: false,
        };

        self.do_request::<(), _>(
            reqwest::Method::POST,
            "/control/tei/register",
            Some(&registration_data),
            None,
        )
        .await
    }

    /// Unregisters a TEI service
    pub async fn unregister_tei_service(
        &self,
        account_id: &str,
        address: &str,
        id: &str,
    ) -> Result<()> {
        let registration_data = RegisterToDiscoveryRequest {
            account_id: account_id.to_string(),
            address: address.to_string(),
            id: id.to_string(),
            is_read: true,
            is_write: false,
        };

        self.do_request::<(), _>(
            reqwest::Method::POST,
            "/control/tei/unregister",
            Some(&registration_data),
            None,
        )
        .await
    }
}
