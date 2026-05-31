use serde::{Deserialize, Serialize};

use crate::events::{HeartBeatEvent, LifeCycleEvent};

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "message_type", rename_all = "lowercase")]
pub enum MessageTypes {
    Private(Box<PrivateMessage>),
    Group(Box<GroupMessage>),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LifeCycleSubType {
    Enable,
    Disable,
    Connect,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HeartbeatStatus {
    pub online: Option<bool>,
    pub good: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "meta_event_type", rename_all = "snake_case")]
pub enum MetaEvent {
    Lifecycle(LifeCycleEvent),
    Heartbeat(HeartBeatEvent),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PrivateMessageSender {
    card: String,
    nickname: String,
    user_id: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum MessageSegment {
    #[serde(rename = "face")]
    Face { id: String },

    #[serde(rename = "text")]
    Text { text: String },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MessageFormat {
    String,
    Array,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Group,
    Private,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GroupMessageSender {
    user_id: u32,
    nickname: String,
    card: String,
    role: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GroupMessage {
    pub echo: Option<String>,
    pub time: u32,
    pub self_id: u32,

    pub real_id: u32,
    pub real_seq: String,
    pub sender: GroupMessageSender,
    pub raw_message: String,
    pub font: u16,
    pub sub_type: String,
    pub message: Vec<MessageSegment>,
    pub message_format: MessageFormat,
    pub group_id: u32,
    pub group_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PrivateMessage {
    pub echo: Option<String>,
    pub time: u32,
    pub self_id: u32,

    pub font: u16,
    pub message: Vec<MessageSegment>,
    pub message_format: MessageFormat,
    pub message_id: u32,
    pub message_seq: u32,
    pub raw_message: String,
    pub real_id: u32,
    pub real_seq: String,
    pub sender: PrivateMessageSender,
    pub sub_type: String,
    pub target_id: u32,
    pub user_id: u32,
}
