use bevy::prelude::*;
use forger::ForgerAppExt;
use forger_discord::DiscordPlugin;
use forger_napcat::{NapcatPlugin, actions::data, bot::NapcatBot, events::PrivateMessageEvent};
use tracing_subscriber::EnvFilter;

fn main() {
    let default_directive = "warn,anya=debug,forger=debug,forger-napcat=debug,forger-discord=debug,twilight-gateway=trace";
    let filter = EnvFilter::new(default_directive);
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let rt_handle = rt.handle();

    let mut app = App::forger_default(rt_handle.clone());
    app.add_plugins(NapcatPlugin);
    app.add_plugins(DiscordPlugin);
    app.world_mut().add_observer(echo_private_msg);

    app.forger_run();
}

fn echo_private_msg(matcher: On<PrivateMessageEvent>, bot: Res<NapcatBot>) {
    let event = matcher.event();
    let user_id = event.user_id;
    let messages = event.message.clone();
    let bot = bot.clone();
    tokio::spawn(async move {
        if let Ok(data::MessageSent { message_id }) = bot.send_private_msg(user_id, messages).await
        {
            dbg!(message_id);
        } else {
            tracing::warn!("error?");
        }
    });
}
