use crate::client::Client;
use crate::error::Result;
use crate::models::HealthResponse;

impl Client {
    /// Performs a health check on the API
    pub async fn health_check(&self) -> Result<HealthResponse> {
        self.do_request::<HealthResponse, ()>(reqwest::Method::GET, "/health", None, None)
            .await
    }
}
