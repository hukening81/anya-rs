use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{
    GroupMessage, GroupMessageSender, HeartbeatStatus, LifeCycleSubType, MessageFormat,
    MessageSegment, PrivateMessage, PrivateMessageSender,
};

// use crate::napcat::{
//     adapter::models::meta_event::{HeartbeatStatus, LifeCycleSubType},
//     models::messages::{GroupMessageSender, MessageFormat, MessageSegment, PrivateMessageSender},
// };

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
