use mvdis_license_plate_query::options::{PlateType, PlateVer, Station};

#[derive(Debug, serde::Deserialize)]
#[serde(default)]
pub struct ClientConfig {
    pub retry_times: usize,
    pub concurrency: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            retry_times: 3,
            concurrency: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(default)]
pub struct TelegramConfig {
    pub message_prefix: String,
    pub bot_token: String,
    pub chat_id: String,
}

impl Default for TelegramConfig {
    fn default() -> Self {
        Self {
            message_prefix: "\\[MVDIS License Plate Bot\\]".to_string(),
            bot_token: "".to_string(),
            chat_id: "".to_string(),
        }
    }
}

fn default_ntfy_server_url() -> String {
    "https://ntfy.sh".to_string()
}

#[derive(Debug, serde::Deserialize)]
pub struct NtfyConfig {
    #[serde(default = "default_ntfy_server_url")]
    pub server_url: String,
    pub topic: String,
}

#[derive(Debug, serde::Deserialize, Default)]
pub struct NotificationConfig {
    #[serde(default)]
    pub telegram: Option<TelegramConfig>,
    #[serde(default)]
    pub ntfy: Option<NtfyConfig>,
}

#[derive(Debug, serde::Deserialize)]
pub struct QueryConfig {
    pub plate_ver: PlateVer,
    pub plate_type: PlateType,
    #[serde(default)]
    pub stations: Option<Vec<Station>>,
    pub matchs: Vec<String>,
}

#[derive(Debug, serde::Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub client: ClientConfig,
    #[serde(default)]
    pub notification: NotificationConfig,
    #[serde(default)]
    pub query: Vec<QueryConfig>,
}

pub fn load_config() -> anyhow::Result<Config> {
    let config_str = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}
