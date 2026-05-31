use std::str::FromStr;

use bevy::prelude::*;
pub struct ForgerConfigPlugin;

static CONFIG_FILE: &str = "forger-config.json";

impl Plugin for ForgerConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ForgerRawConfig::default());
    }
}

#[derive(Resource, Deref)]
pub struct ForgerRawConfig(pub serde_json::Value);

impl Default for ForgerRawConfig {
    fn default() -> Self {
        let content = std::fs::read_to_string(CONFIG_FILE).expect("Unable to read config file");
        let value = serde_json::Value::from_str(&content).expect("Unable to parse config");

        Self(value)
    }
}
