use crate::{
    api::{Command, Message},
    UnsafeSend,
};
use log::{debug, error, info};
use mpv::MpvHandler;
use std::sync::Arc;
use tokio::sync::{mpsc::Receiver, Mutex};

pub fn configure_player() -> MpvHandler {
    log::info!("Configuring MPV Player");
    let mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");
    mpv_builder.build().expect("failed to build")
}

pub async fn main(rx: Arc<Mutex<Receiver<Message>>>, player: Arc<Mutex<UnsafeSend<MpvHandler>>>) {
    log::info!("Loading file into MPV Player");
    loop {
        if let Ok(message) = rx.clone().lock().await.try_recv() {
            match message.ipc_command {
                Command::Start => {
                    let mut locked_player = player.lock().await;
                    let command_array = [
                        "loadfile",
                        "/home/ad/.zenpi/application_data/audio_files/delta.m4a",
                    ];
                    let _ = match locked_player.command_async(&command_array, 0) {
                        Ok(_) => {
                            info!("Start command has been recieved");
                            loop {
                                match locked_player.wait_event(1.0) {
                                    Some(mpv_event) => match mpv_event {
                                        mpv::Event::PlaybackRestart => {
                                            info!("Playback has started successfully");
                                            break;
                                        },
                                        mpv::Event::EndFile(env_file_reason) => match env_file_reason {
                                            Ok(_) => log::debug!("Start command has been sent!"),
                                            Err(err) => {
                                                error!("{}", err);
                                                break;
                                            }
                                        },
                                        some_other_event => {
                                            debug!("Event: {:?}", some_other_event);
                                        },
                                    },
                                    None => (),
                                }
                            } 
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    };
                }

                Command::Stop => {
                    let command_array = ["stop"];
                    let _ = player
                        .lock()
                        .await
                        .command(&command_array)
                        .expect("Player to stop playing.");
                    info!("ZenPi has stopped playing.");
                }
            }
        }
    }
}
