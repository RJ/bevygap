use bevy::prelude::*;

/// Represents the environment variables provided by Arbitrium for deployments.
#[derive(Debug, Clone, Resource)]
pub struct ArbitriumEnv {
    /// Your deployment request ID. This is a unique ID across all Arbitrium. Can be used to retrieve information.
    pub request_id: String,
    /// URL to call to delete your deployment from within itself. Visit the API documentation for more details about this route.
    pub delete_url: String,
    /// Authorization token to call ARBITRIUM_DELETE_URL.
    pub delete_token: String,
    /// JSON encoded string that contains data about the location of your deployment.
    pub deployment_location: String,
    /// URL to get the context of your deployment. Visit the API documentation for more details about this route.
    pub context_url: String,
    /// Authorization token to call ARBITRIUM_CONTEXT_URL.
    pub context_token: String,
    /// The public IP of your deployment.
    pub public_ip: String,
    /// JSON string of the ports mapping of your deployment.
    pub ports_mapping: String,
}

impl ArbitriumEnv {
    /// Creates a new instance of `ArbitriumEnv` from environment variables.
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            request_id: std::env::var("ARBITRIUM_REQUEST_ID")?,
            delete_url: std::env::var("ARBITRIUM_DELETE_URL")?,
            delete_token: std::env::var("ARBITRIUM_DELETE_TOKEN")?,
            deployment_location: std::env::var("ARBITRIUM_DEPLOYMENT_LOCATION")?,
            context_url: std::env::var("ARBITRIUM_CONTEXT_URL")?,
            context_token: std::env::var("ARBITRIUM_CONTEXT_TOKEN")?,
            public_ip: std::env::var("ARBITRIUM_PUBLIC_IP")?,
            ports_mapping: std::env::var("ARBITRIUM_PORTS_MAPPING")?,
        })
    }

    /// Returns a tuple containing the request_id and security_number extracted from the context_url.
    /// The security_number is parsed as an i32.
    pub fn context_parts(&self) -> Option<(String, i32)> {
        let parts: Vec<&str> = self.context_url.split('/').collect();
        if parts.len() >= 2 {
            let security_number = parts.last().and_then(|s| s.parse::<i32>().ok())?;
            let request_id = parts[parts.len() - 2].to_string();
            Some((request_id, security_number))
        } else {
            None
        }
    }
}
