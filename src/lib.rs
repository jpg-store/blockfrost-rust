//! Rust SDK for Blockfrost.io

pub mod models;
pub mod settings;
pub mod env;

pub const CARDANO_MAINNET_NETWORK: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
pub const CARDANO_TESTNET_NETWORK: &str = "https://cardano-testnet.blockfrost.io/api/v0";

pub use settings::Settings;

#[derive(Debug, Clone)]
pub struct BlockFrostApi {
    settings: Settings,
    client: reqwest::Client,
}

impl BlockFrostApi {
    pub fn new(settings: Settings) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();

        let project_id = settings.project_id.parse().expect("Should be ??");

        headers.insert("project_id", project_id);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            settings,
            client,
        }
    }

    pub async fn health(&self) -> reqwest::Result<models::Health> {
        let suffix = "/health";
        let full_url = self.settings.network_endpoint.to_string() + suffix;
        dbg!(&full_url);
        let response = self.client.get(full_url).send().await?;
        dbg!(&response);
        response.json().await
    }
}
