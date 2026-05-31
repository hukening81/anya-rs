use bevy::prelude::*;

use crate::config::napcat_config_plugin;

mod adapter;
pub mod config;
pub mod events;
pub mod models;

pub use adapter::*;

pub struct NapcatPlugin;

impl Plugin for NapcatPlugin {
    fn build(&self, app: &mut App) {
        tracing::info!("Hello from napcat");
        app.add_plugins(napcat_config_plugin);
        app.add_plugins(AdapterPlugin {});
    }
}
