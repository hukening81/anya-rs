use bevy::{app::App, prelude::Resource};
use forger::ForgerAppExt;
use serde::Deserialize;

pub fn napcat_config_plugin(app: &mut App) {
    let raw_config = app.get_raw_config();
    let napcat = NapcatConfig::from_raw_config(raw_config);

    app.insert_resource(napcat);
}

#[derive(Deserialize, Debug, Clone, Resource)]
pub struct NapcatConfig {
    pub adapter: NapcatAdapterConfig,
}

impl NapcatConfig {
    pub fn from_raw_config(value: &serde_json::Value) -> NapcatConfig {
        let napcat = value
            .get("napcat")
            .expect("Failed to get napcat's config section");
        let napcat = (*napcat).clone();

        let napcat: NapcatConfig =
            serde_json::from_value(napcat).expect("Failed to parse napcat config");

        napcat
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct NapcatAdapterConfig {
    pub url: String,
    pub token: String,
}
impl NapcatAdapterConfig {
    pub fn ws_address_with_token(&self) -> String {
        format!("{}?access_token={}", self.url, self.token)
    }
}
