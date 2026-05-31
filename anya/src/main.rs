use bevy::prelude::*;
use forger::ForgerAppExt;
use forger_napcat::{
    NapcatActionParams, NapcatActionRequest, NapcatClientAction, NapcatClientActionSender,
    NapcatPlugin, events::PrivateMessageEvent,
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
fn echo_test(matcher: On<PrivateMessageEvent>, action_sender: Res<NapcatClientActionSender>) {
    let event = matcher.event();
    let user_id = event.user_id;
    let messages = event.message.clone();
    let action_sender = action_sender.clone();
    tokio::spawn(async move {
        let (tx, rx) = forger_napcat::create_client_action_channel();

        let context = NapcatClientAction {
            result_sender: tx,
            action: NapcatActionRequest {
                echo: uuid::Uuid::new_v4().into(),
                action: NapcatActionParams::SendPrivateMsg {
                    user_id,
                    message: messages,
                },
            },
        };
        if let Ok(()) = action_sender.send(context) {
            if let Ok(response) = rx.await {
                dbg!(response);
            }
        } else {
            tracing::warn!("error?");
        }
    });
}
