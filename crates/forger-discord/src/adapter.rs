use crate::config::{DiscordAdapterConfig, DiscordConfig};
use bevy::app::App;
use forger::ForgerAppExt;
use twilight_gateway::{Config, ConfigBuilder, EventTypeFlags, Shard, StreamExt};
use twilight_model::gateway::{Intents, ShardId};

pub fn discord_adapter_plugin(app: &mut App) {
    let config = app
        .world()
        .get_resource::<DiscordConfig>()
        .unwrap()
        .adapter
        .clone();

    let rt_handle = app.get_tokio_runtime_handle();
    rt_handle.spawn(async {
        let mut adapter = AdapterBuilder::new(config).build();
        adapter.run().await;
    });
}

struct Adapter {
    shard: Shard,
}

impl Adapter {
    pub fn new(config: DiscordAdapterConfig) -> Self {
        let intents = Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT;
        let mut config_builder = ConfigBuilder::new(config.bot_token.clone(), intents);
        if let Some(proxy_url) = config.proxy {
            config_builder = config_builder.proxy_url(proxy_url);
        }

        let shard = Shard::with_config(ShardId::ONE, config_builder.build());

        Self { shard }
    }
    async fn run(&mut self) {
        while let Some(item) = self.shard.next_event(EventTypeFlags::all()).await {
            if let Ok(event) = item {
                dbg!(event);
            } else {
                dbg!(item);
                dbg!(self.shard.state());
            }
        }
    }
}

struct AdapterBuilder {
    config: DiscordAdapterConfig,
}

impl AdapterBuilder {
    pub fn new(config: DiscordAdapterConfig) -> Self {
        Self { config }
    }
    pub fn build(self) -> Adapter {
        Adapter::new(self.config)
    }
}
