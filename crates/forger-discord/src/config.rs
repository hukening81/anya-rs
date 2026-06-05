use anyhow::Context;
use bevy::{app::App, ecs::resource::Resource};
use forger::ForgerAppExt;
use serde::Deserialize;

pub fn discord_config_plugin(app: &mut App) {
    app.insert_resource(DiscordConfig::new(app).unwrap());
}

#[derive(Deserialize, Resource, Clone, Debug)]
pub struct DiscordConfig {
    pub adapter: DiscordAdapterConfig,
}

impl DiscordConfig {
    pub fn new(app: &App) -> anyhow::Result<Self> {
        let raw_config = app.get_raw_config();
        let value = raw_config
            .get("discord")
            .context("Failed to get discord section of the config")?
            .clone();
        let config: Self =
            serde_json::from_value(value).context("Failed to parse discord config")?;
        Ok(config)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct DiscordAdapterConfig {
    pub proxy: Option<String>,
    pub bot_token: String,
}
