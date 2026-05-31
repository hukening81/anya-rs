use serde::{Deserialize, Serialize};

use crate::{NapcatActionResultReceiverType, NapcatClientAction};

pub mod params {
    use serde::Serialize;

    use crate::{actions::NapcatAction, models::MessageSegment};

    #[derive(Serialize, Clone)]
    pub struct GetGroupInfo {
        pub group_id: u32,
    }
    impl From<GetGroupInfo> for NapcatAction {
        fn from(value: GetGroupInfo) -> Self {
            Self::GetGroupInfo(value)
        }
    }

    #[derive(Serialize, Clone)]
    pub struct SendPrivateMsg {
        pub user_id: u32,
        pub message: Vec<MessageSegment>,
    }
    impl From<SendPrivateMsg> for NapcatAction {
        fn from(value: SendPrivateMsg) -> Self {
            Self::SendPrivateMsg(value)
        }
    }

    #[derive(Serialize, Clone)]
    pub struct SendGroupMsg {
        pub group_id: u32,
        pub message: Vec<MessageSegment>,
    }
    impl From<SendGroupMsg> for NapcatAction {
        fn from(value: SendGroupMsg) -> Self {
            Self::SendGroupMsg(value)
        }
    }
}

#[derive(Serialize, Clone)]
#[serde(tag = "action", content = "params", rename_all = "snake_case")]
pub enum NapcatAction {
    GetLoginInfo,
    GetGroupInfo(params::GetGroupInfo),
    SendPrivateMsg(params::SendPrivateMsg),
    SendGroupMsg(params::SendGroupMsg),
}

#[derive(Serialize, Clone)]
pub struct NapcatActionRequest {
    #[serde(flatten)]
    pub action: NapcatAction,

    pub echo: String,
}

impl NapcatActionRequest {
    pub fn create_context(
        action: NapcatAction,
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
