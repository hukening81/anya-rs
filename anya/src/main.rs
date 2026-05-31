use bevy::prelude::*;
use forger::ForgerAppExt;
use forger_napcat::{
    NapcatPlugin,
    actions::data,
    bot::NapcatBot,
    events::PrivateMessageEvent,
};
use tracing::Level;

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let rt_handle = rt.handle();

    let mut app = App::forger_default(rt_handle.clone());
    app.add_plugins(NapcatPlugin);
    app.world_mut().add_observer(echo_test);

    app.forger_run();
}
fn echo_test(matcher: On<PrivateMessageEvent>, bot: Res<NapcatBot>) {
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
