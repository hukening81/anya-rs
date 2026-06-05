use crate::{adapter::discord_adapter_plugin, config::discord_config_plugin};
use bevy::app::Plugin;

mod adapter;
pub mod config;

pub struct DiscordPlugin;

impl Plugin for DiscordPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        tracing::info!("hello from discord");
        app.add_plugins(discord_config_plugin);
        app.add_plugins(discord_adapter_plugin);
    }
}

struct DiscordApiBuilder {
    base_api: String,
    segments: Vec<String>,
}
impl DiscordApiBuilder {
    pub fn new() -> Self {
        Self {
            base_api: "https://discord.com/api/".into(),
            segments: vec![],
        }
    }
    pub fn add_segment(mut self, segment: &str) -> Self {
        let segment = segment.trim_start_matches("/");
        let segment = segment.trim_end_matches("/");
        self.segments.push(segment.into());
        self
    }
    pub fn build(self) -> String {
        self.into()
    }
}

impl From<DiscordApiBuilder> for String {
    fn from(value: DiscordApiBuilder) -> Self {
        format!("{}{}", value.base_api, value.segments.join("/"))
    }
}
