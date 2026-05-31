use bevy::prelude::*;
use serde::Deserialize;

use crate::models::{
    GroupMessage, HeartbeatStatus, LifeCycleSubType, MessageTypes, MetaEvent, PrivateMessage,
};

#[derive(Deref, Clone, Event)]
pub struct GroupMessageEvent(pub GroupMessage);

#[derive(Deref, Clone, Event)]
pub struct PrivateMessageEvent(pub PrivateMessage);

#[derive(Deref, Clone, Event)]
pub struct GroupMessageSentEvent(pub GroupMessage);

#[derive(Deref, Clone, Event)]
pub struct PrivateMessageSentEvent(pub PrivateMessage);

#[derive(Deserialize, Debug, Clone, Event)]
pub struct LifeCycleEvent {
    pub sub_type: LifeCycleSubType,
}

#[derive(Deserialize, Debug, Clone, Event)]
pub struct HeartBeatEvent {
    pub status: HeartbeatStatus,
    pub interval: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "post_type", rename_all = "snake_case")]
pub enum NapcatEvent {
    Message(Box<MessageTypes>),

    #[serde(rename = "meta_event")]
    Meta(Box<MetaEvent>),

    Request,

    Notice,

    MessageSent(Box<MessageTypes>),
}
