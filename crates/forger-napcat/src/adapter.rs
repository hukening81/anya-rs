use std::{sync::Arc, time::Duration};

use bevy::{
    app::{Plugin, PreUpdate},
    ecs::{
        resource::Resource,
        system::{Commands, ResMut},
    },
    platform::collections::HashMap,
    prelude::{Deref, DerefMut},
};
use forger::{ForgerAppExt, WakeupSenderType};
use futures::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};

use crate::{
    actions::{NapcatActionRequest, NapcatActionResponse},
    config::{NapcatAdapterConfig, NapcatConfig},
    events::{
        GroupMessageEvent, GroupMessageSentEvent, NapcatEvent, PrivateMessageEvent,
        PrivateMessageSentEvent,
    },
    models::{MessageTypes, MetaEvent},
};

pub type NapcatActionResultSenderType = tokio::sync::oneshot::Sender<NapcatActionResponse>;
pub type NapcatActionResultReceiverType = tokio::sync::oneshot::Receiver<NapcatActionResponse>;

pub struct NapcatClientAction {
    pub result_sender: NapcatActionResultSenderType,
    pub action: NapcatActionRequest,
}

pub type NapcatClientActionSenderType = tokio::sync::mpsc::UnboundedSender<NapcatClientAction>;
pub type NapcatClientActionReceiverType = tokio::sync::mpsc::UnboundedReceiver<NapcatClientAction>;

type ActionStoreType = Arc<tokio::sync::Mutex<HashMap<String, NapcatActionResultSenderType>>>;

type ClientEventSenderType = tokio::sync::mpsc::UnboundedSender<NapcatEvent>;
type ClientEventReceiverType = tokio::sync::mpsc::UnboundedReceiver<NapcatEvent>;

fn populate_napcat_event(
    mut event_receiver: ResMut<NapcatClientEventReceiver>,
    mut commands: Commands,
) {
    while let Ok(event) = event_receiver.try_recv() {
        match event {
            NapcatEvent::Message(message_event) => match *message_event {
                MessageTypes::Private(private_message) => {
                    tracing::debug!("Triggering event [message_event:private]");
                    commands.trigger(PrivateMessageEvent(*private_message));
                }
                MessageTypes::Group(group_message) => {
                    tracing::debug!("Triggering event [message_event:group]");
                    commands.trigger(GroupMessageEvent(*group_message));
                }
            },
            NapcatEvent::Meta(meta_event) => match *meta_event {
                MetaEvent::Lifecycle(life_cycle_event) => {
                    tracing::debug!("Triggering event [meta_event:lifecycle]");
                    commands.trigger(life_cycle_event);
                }
                MetaEvent::Heartbeat(heart_beat_event) => {
                    tracing::debug!("Triggering event [meta_event:heart_beat_event]");
                    commands.trigger(heart_beat_event);
                }
            },
            NapcatEvent::Request => {
                tracing::warn!("new event [Request] will not be triggered")
            }
            NapcatEvent::Notice => {
                tracing::warn!("new event [Notice] will not be triggered")
            }
            NapcatEvent::MessageSent(message) => match *message {
                MessageTypes::Private(private_message) => {
                    tracing::debug!("Triggering event [message_sent_event:private]");
                    commands.trigger(PrivateMessageSentEvent(*private_message));
                }
                MessageTypes::Group(group_message) => {
                    tracing::debug!("Triggering event [message_sent_event:private]");
                    commands.trigger(GroupMessageSentEvent(*group_message));
                }
            },
        }
    }
}

pub struct AdapterPlugin {}

impl Plugin for AdapterPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        tracing::info!("Setting up napcat adapter");
        let adapter_config = app.world().get_resource::<NapcatConfig>().unwrap();
        let wakeup_sender = app.get_wakeup_sender();

        let (adapter, client_event_receiver) =
            NapcatAdapter::new(adapter_config.adapter.clone(), wakeup_sender);

        app.insert_resource(NapcatClientActionSender(
            adapter.client_action_sender.clone(),
        ));
        app.insert_resource(NapcatClientEventReceiver(client_event_receiver));
        app.add_systems(PreUpdate, populate_napcat_event);

        app.get_tokio_runtime_handle().spawn(adapter.run());
    }
}

#[derive(Resource, DerefMut, Deref)]
pub struct NapcatClientActionSender(pub NapcatClientActionSenderType);

#[derive(Resource, DerefMut, Deref)]
pub struct NapcatClientEventReceiver(pub ClientEventReceiverType);

struct NapcatAdapter {
    config: NapcatAdapterConfig,
    action_store: ActionStoreType,
    wakeup_sender: WakeupSenderType,

    client_action_receiver: NapcatClientActionReceiverType,
    pub client_action_sender: NapcatClientActionSenderType,
    pub client_event_sender: ClientEventSenderType,
}

impl NapcatAdapter {
    pub fn new(
        config: NapcatAdapterConfig,
        wakeup_sender: WakeupSenderType,
    ) -> (Self, ClientEventReceiverType) {
        let (client_event_sender, client_event_receiver) = tokio::sync::mpsc::unbounded_channel();
        let (client_action_sender, client_action_receiver) = tokio::sync::mpsc::unbounded_channel();

        let instance = Self {
            config,
            action_store: ActionStoreType::default(),
            wakeup_sender,
            client_action_receiver,
            client_event_sender,
            client_action_sender,
        };
        (instance, client_event_receiver)
    }
    pub async fn run(mut self) {
        loop {
            tracing::info!("Connecting to napcat {}", self.config.url);
            let (stream, _) = match connect_async(self.config.ws_address_with_token()).await {
                std::result::Result::Ok(v) => v,
                Err(e) => {
                    tracing::info!("Failed when connecting napcat {:?}", e);
                    tokio::time::sleep(Duration::from_secs(3)).await;
                    continue;
                }
            };

            let (ws_tx, ws_rx) = stream.split();
            tokio::select! {
                _ = Self::handle_websocket_frames(
                    self.action_store.clone(),
                    self.client_event_sender.clone(),self.wakeup_sender.clone(),ws_rx) => {
                    println!("Event handler loop exited.");
                }
                _ = Self::handle_actions(
                    &mut self.client_action_receiver,
                    self.action_store.clone(),ws_tx
                ) => {
                    println!("Action handler loop exited.");
                }

            }
        }
    }
    async fn handle_websocket_frames(
        action_store: ActionStoreType,
        event_sender: ClientEventSenderType,
        wakeup_sender: WakeupSenderType,
        mut ws_reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) {
        while let Some(message_result) = ws_reader.next().await {
            match message_result {
                std::result::Result::Ok(message) => match message {
                    tokio_tungstenite::tungstenite::Message::Text(text) => {
                        tracing::info!("Received new frame: {}", text);
                        match serde_json::from_str::<NapcatWebsocketFrame>(&text) {
                            std::result::Result::Ok(e) => match e {
                                NapcatWebsocketFrame::Event(event_wrapper) => {
                                    event_sender.send(event_wrapper.clone()).unwrap();
                                    let _ = wakeup_sender.send("napcat event".into());
                                }
                                NapcatWebsocketFrame::ActionResponse(action_response_wrapper) => {
                                    let mut guard = action_store.lock().await;
                                    if let Some(sender) =
                                        guard.remove(&action_response_wrapper.echo)
                                    {
                                        let _ = sender.send(action_response_wrapper);
                                    } else {
                                        tracing::warn!(
                                            "Napcat sent a api response but there is not a correspond request {:?}",
                                            action_response_wrapper
                                        );
                                    }
                                }
                            },
                            Err(e) => {
                                println!("Failed to parse event: {}\n\t{}", e, text);
                            }
                        }
                    }
                    tokio_tungstenite::tungstenite::Message::Binary(bytes) => {
                        println!("{:?}", bytes)
                    }
                    tokio_tungstenite::tungstenite::Message::Ping(bytes) => {
                        tracing::info!("Ping {:?}", bytes);
                    }
                    tokio_tungstenite::tungstenite::Message::Pong(bytes) => {
                        tracing::info!("Pong {:?}", bytes);
                    }
                    tokio_tungstenite::tungstenite::Message::Close(close_frame) => {
                        tracing::info!("Close {:?}", close_frame)
                    }
                    tokio_tungstenite::tungstenite::Message::Frame(frame) => {
                        tracing::info!("Frame {:?}", frame)
                    }
                },
                Err(e) => {
                    tracing::warn!("Encounter error {:?}", e)
                }
            }
        }
        //TODO: handle None situations
    }

    async fn handle_actions(
        client_action_receiver: &mut NapcatClientActionReceiverType,
        action_store: ActionStoreType,
        mut ws_sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    ) {
        loop {
            if let Some(NapcatClientAction {
                action,
                mut result_sender,
            }) = client_action_receiver.recv().await
            {
                match serde_json::to_string(&action.clone()) {
                    Ok(text) => {
                        {
                            let mut guard = action_store.lock().await;
                            guard.insert(action.echo, result_sender);
                            drop(guard);
                        }

                        let _ = ws_sender.send(Message::text(text)).await;
                    }
                    Err(e) => {
                        tracing::warn!("{:?}", e);
                        let _ = result_sender.closed().await;
                    }
                };
            };
        }
    }
}

// pub type NapcatClientEventSenderType = tokio::sync::mpsc::UnboundedSender<>

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum NapcatWebsocketFrame {
    Event(NapcatEvent),
    ActionResponse(NapcatActionResponse),
}
