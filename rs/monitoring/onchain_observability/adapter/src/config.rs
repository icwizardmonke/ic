use ic_config::{
    crypto::CryptoConfig, logger::Config as LoggerConfig, registry_client::Config as RegistryConfig,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationSeconds};
use std::time::Duration;

/// This struct contains configuration options for the observability canister adapter
/// It is a combination of Onchain Observability Adapter Specific Config and the IC Replica Config
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct Config {
    pub logger: LoggerConfig,
    pub crypto_config: CryptoConfig,
    pub registry_config: RegistryConfig,
    pub report_length_sec: Duration,
    pub sampling_interval_sec: Duration,
    pub canister_client_url: String,
    pub canister_id: String,
    pub uds_socket_path: String,
}

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct OnchainObservabilityAdapterSpecificConfig {
    pub logger: LoggerConfig,
    #[serde_as(as = "DurationSeconds<u64>")]
    #[serde(default = "default_report_length")]
    pub report_length_sec: Duration,
    #[serde_as(as = "DurationSeconds<u64>")]
    #[serde(default = "default_sampling_interval")]
    pub sampling_interval_sec: Duration,
    #[serde(default = "default_url")]
    pub canister_client_url: String,
    pub canister_id: String,
    #[serde(default = "uds_default_socket")]
    pub uds_socket_path: String,
}

pub(crate) const fn default_report_length() -> Duration {
    Duration::from_secs(3600) // 1 hour
}

pub(crate) const fn default_sampling_interval() -> Duration {
    Duration::from_secs(60) // 1 minute
}

pub(crate) fn default_url() -> String {
    "https://ic0.app".to_string()
}

pub(crate) fn uds_default_socket() -> String {
    "/run/ic-node/onchain-observability-adapter/socket".to_string()
}
