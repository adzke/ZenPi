use std::sync::Arc;
use mpv::MpvHandler;
use tokio::sync::{mpsc::Receiver, Mutex};
use log::info;
use crate::{
    api::{Command, Message},
    UnsafeSend,
};

pub fn configure_player() -> MpvHandler {
    log::info!("Configuring MPV Player");
    let mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");
    return mpv_builder.build().expect("failed to build");
}

pub async fn main(rx: Arc<Mutex<Receiver<Message>>>, player: Arc<Mutex<UnsafeSend<MpvHandler>>>) {
    log::info!("Loading file into MPV Player");

    loop {
        if let Ok(message) = rx.clone().lock().await.try_recv() {
            match message.ipc_command {
                Command::Start => {
                    let command_array = ["loadfile", "/home/ad/Downloads/delta.m4a"];
                    let _ = player
                        .lock()
                        .await
                        .command(&command_array)
                        .expect("Failed to execute command");
                    info!("ZenPi has started playing.");

                }
                Command::Stop => {
                    let command_array = ["stop"];
                    let _ = player
                        .lock()
                        .await
                        .command(&command_array)
                        .expect("Failed to execute command");
                    info!("ZenPi has stopped playing.");
                }
            }
        }
    }
}
