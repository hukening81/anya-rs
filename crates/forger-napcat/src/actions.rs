use serde::{Deserialize, Serialize};

use crate::{NapcatActionResultReceiverType, NapcatClientAction, models::MessageSegment};

#[derive(Serialize, Clone)]
#[serde(tag = "action", content = "params", rename_all = "snake_case")]
pub enum NapcatActionParams {
    GetLoginInfo,
    GetGroupInfo {
        group_id: u32,
    },
    SendPrivateMsg {
        user_id: u32,
        message: Vec<MessageSegment>,
    },
    SendGroupMsg {
        group_id: u32,
        message: Vec<MessageSegment>,
    },
}

#[derive(Serialize, Clone)]
pub struct NapcatActionRequest {
    #[serde(flatten)]
    pub action: NapcatActionParams,

    pub echo: String,
}

impl NapcatActionRequest {
    pub fn create_context(
        action: NapcatActionParams,
    ) -> (NapcatClientAction, NapcatActionResultReceiverType) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let request = NapcatActionRequest {
            echo: uuid::Uuid::new_v4().into(),
            action,
        };
        (
            NapcatClientAction {
                result_sender: tx,
                action: request,
            },
            rx,
        )
    }
}

pub mod data {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    pub struct GetLoginInfo {
        pub user_id: u32,
        pub nickname: String,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct GetGroupInfo {
        pub group_all_shut: u32,
        pub group_remark: String,
        pub group_id: u32,
        pub group_name: String,
        pub member_count: u32,
        pub max_member_count: u32,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct MessageSent {
        pub message_id: u32,
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ActionResponseInner {
    GetLoginInfo(data::GetLoginInfo),
    GetGroupInfo(data::GetGroupInfo),
    MessageSent(data::MessageSent),
}

#[derive(Deserialize, Debug, Clone)]
pub struct NapcatActionResponse {
    pub status: String,
    pub retcode: u32,
    pub data: ActionResponseInner,
    pub message: String,
    pub wording: String,
    pub echo: String,
    pub stream: String,
}
