use std::sync::Arc;
use mpv::MpvHandler;
use tokio::sync::{mpsc::Receiver, Mutex};
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
            println!("{:?}", message.ipc_command);
            match message.ipc_command {
                Command::Start => {
                    let command_array = ["loadfile", "/home/ad/Downloads/delta.m4a"];
                    let _ = player
                        .lock()
                        .await
                        .command(&command_array)
                        .expect("Failed to execute command");
                }
                Command::Stop => {
                    let command_array = ["stop"];
                    let _ = player
                        .lock()
                        .await
                        .command(&command_array)
                        .expect("Failed to execute command");
                }
                Command::TimeRemaining => {
                    let command_array = ["time-remaining"];
                    let _ = player
                        .lock()
                        .await
                        .command(&command_array)
                        .expect("Failed to execute command");
                }
            }
        }
    }
}
