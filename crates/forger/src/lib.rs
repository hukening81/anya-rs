use bevy::prelude::*;

use crate::config::{ForgerConfigPlugin, ForgerRawConfig};

pub mod config;

pub struct ForgerPlugin;

impl Plugin for ForgerPlugin {
    fn build(&self, app: &mut App) {
        let rt_handle = app
            .world()
            .get_resource::<TokioRuntimeHandle>()
            .expect("Unable to get tokio runtime handle");

        let rt_handle = (*rt_handle).clone();

        let (wakeup_tx, wakeup_rx) = tokio::sync::broadcast::channel::<WakeupContextType>(64);
        app.insert_resource(TokioRuntimeHandle(rt_handle))
            .insert_resource(WakeupSender(wakeup_tx))
            .insert_resource(WakeupReceiver(wakeup_rx));
    }
}

pub trait ForgerAppExt {
    fn forger_default(rt_handle: tokio::runtime::Handle) -> Self;
    fn forger_run(&mut self);
    fn handle_wakeup(&mut self) -> impl Future<Output = ()>;

    fn get_raw_config(&self) -> &ForgerRawConfig;
    fn get_wakeup_sender(&self) -> WakeupSenderType;
    fn get_tokio_runtime_handle(&self) -> tokio::runtime::Handle;
}

impl ForgerAppExt for App {
    fn forger_default(rt_handle: tokio::runtime::Handle) -> Self {
        let mut app = bevy::app::App::new();

        app.add_plugins(MinimalPlugins);
        app.insert_resource(TokioRuntimeHandle(rt_handle));
        app.add_plugins(ForgerConfigPlugin);
        app.add_plugins(ForgerPlugin);

        app
    }

    fn forger_run(&mut self) {
        let rt_handle = self
            .world()
            .get_resource::<TokioRuntimeHandle>()
            .expect("Unable to retrieve tokio runtime handle");
        let rt_handle = (*rt_handle).clone();
        rt_handle.block_on(self.handle_wakeup());
    }

    async fn handle_wakeup(&mut self) {
        let wakeup_receiver = self
            .world()
            .get_resource::<WakeupReceiver>()
            .expect("Unable to get wakeup receiver");
        let mut wakeup_receiver = wakeup_receiver.0.resubscribe();
        while let Ok(context) = wakeup_receiver.recv().await {
            tracing::trace!(
                "Triggering a bevy update, reason for this update: {}",
                context
            );
            self.update();
        }
    }

    fn get_raw_config(&self) -> &ForgerRawConfig {
        self.world()
            .get_resource::<ForgerRawConfig>()
            .expect("Forger config is not loaded")
    }

    fn get_wakeup_sender(&self) -> WakeupSenderType {
        (*self
            .world()
            .get_resource::<WakeupSender>()
            .expect("Failed to get wakeup sender"))
        .clone()
    }

    fn get_tokio_runtime_handle(&self) -> tokio::runtime::Handle {
        self.world()
            .get_resource::<TokioRuntimeHandle>()
            .unwrap()
            .0
            .clone()
    }
}

pub type WakeupContextType = String;
pub type WakeupSenderType = tokio::sync::broadcast::Sender<WakeupContextType>;
pub type WakeupReceiverType = tokio::sync::broadcast::Receiver<WakeupContextType>;

#[derive(Resource, DerefMut, Deref)]
pub struct WakeupSender(pub WakeupSenderType);

#[derive(Resource, DerefMut, Deref)]

pub struct WakeupReceiver(pub WakeupReceiverType);

#[derive(Resource, DerefMut, Deref)]
pub struct TokioRuntimeHandle(pub tokio::runtime::Handle);
