use bevy::{app::App, ecs::resource::Resource};

use crate::{
    NapcatClientActionSender, NapcatClientActionSenderType,
    actions::{self, NapcatActionRequest},
    models::MessageSegment,
};

pub fn napcat_bot_plugin(app: &mut App) {
    app.insert_resource(NapcatBot::new(app));
}

#[derive(Resource, Clone)]
pub struct NapcatBot {
    action_sender: NapcatClientActionSenderType,
}

impl NapcatBot {
    fn new(app: &App) -> Self {
        let action_sender = app
            .world()
            .get_resource::<NapcatClientActionSender>()
            .unwrap();
        let action_sender = (*action_sender).clone();
        Self { action_sender }
    }
}
impl NapcatBot {
    pub async fn get_login_info(&self) -> anyhow::Result<actions::data::GetLoginInfo> {
        let (context, rx) =
            NapcatActionRequest::create_context(actions::NapcatActionParams::GetLoginInfo);
        self.action_sender.send(context)?;
        let response = rx.await?;
        if let actions::ActionResponseInner::GetLoginInfo(data) = response.data {
            Ok(data)
        } else {
            anyhow::bail!("Action response type do not match")
        }
    }
    pub async fn send_private_msg(
        &self,
        user_id: u32,
        message: Vec<MessageSegment>,
    ) -> anyhow::Result<actions::data::MessageSent> {
        let (context, rx) =
            NapcatActionRequest::create_context(actions::NapcatActionParams::SendPrivateMsg {
                user_id,
                message,
            });
        self.action_sender.send(context)?;
        let response = rx.await?;
        if let actions::ActionResponseInner::MessageSent(data) = response.data {
            Ok(data)
        } else {
            anyhow::bail!("Action response type do not match")
        }
    }
}
