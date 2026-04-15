use crate::client::Client;
use crate::error::Result;
use crate::models::{
    GenericResponse, GetSettingsResponse, SettingsAvailableProvidersResponse, SettingsUpdateRequest,
};

impl Client {
    /// Retrieves the server settings
    pub async fn get_settings(&self) -> Result<GetSettingsResponse> {
        self.do_request::<GetSettingsResponse, ()>(reqwest::Method::GET, "/api/settings/v1/", None, None)
            .await
    }

    /// Updates server settings
    pub async fn update_settings(&self, settings: &SettingsUpdateRequest) -> Result<GenericResponse> {
        self.do_request(
            reqwest::Method::PUT,
            "/api/settings/v1/",
            Some(settings),
            None,
        )
        .await
    }

    /// Lists available auth and integration providers
    pub async fn list_providers(&self) -> Result<SettingsAvailableProvidersResponse> {
        self.do_request::<SettingsAvailableProvidersResponse, ()>(
            reqwest::Method::GET,
            "/api/settings/v1/providers",
            None,
            None,
        )
        .await
    }
}
