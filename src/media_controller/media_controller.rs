use crate::{
    api::{Command, Message},
    file_controller::file_controller::FileController,
    UnsafeSend,
};
use log::{debug, error, info};
use mpv::MpvHandler;
use core::panic;
use std::sync::Arc;
use tokio::sync::{mpsc::Receiver, Mutex};

pub fn configure_player() -> MpvHandler {
    log::info!("Configuring MPV Player");
    let mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");
    mpv_builder.build().expect("failed to build")
}

pub async fn main(
    rx: Arc<Mutex<Receiver<Message>>>,
    player: Arc<Mutex<UnsafeSend<MpvHandler>>>,
    file_controller: Arc<Mutex<FileController>>,
) {
    log::info!("Loading file into MPV Player");
    loop {
        if let Ok(message) = rx.clone().lock().await.try_recv() {
            log::debug!("Messaged received");
            match message.ipc_command {
                Command::Start => {
                    log::debug!("Start Message found");
                    log::debug!("{:?}", message);
                    log::debug!("Play has been locked");

                    let file_controller = file_controller.lock().await;
                    log::debug!("file_controller has been locked");
                    let track_path = {
                        match message.track {
                            Some(track) => track.track_path,
                            None => {
                                log::debug!("No track found in message");
                                match file_controller.default_track.clone() {
                                    Some(track) => {
                                        log::debug!("Default track found");
                                        track.clone().track_path
                                    }
                                    None => {
                                        log::error!("No Default track found");
                                        panic!("No default track found, exiting!")
                                    },
                            }
                            },
                        }
                    };

                    log::debug!("Track path has been defined");
                    let mut locked_player = player.lock().await;

                    let command_array = ["loadfile", track_path.to_str().expect("expect str")];
                    let _ = match locked_player.command_async(&command_array, 0) {
                        Ok(_) => {
                            info!("Start command has been recieved");
                            loop {
                                match locked_player.wait_event(1.0) {
                                    Some(mpv_event) => match mpv_event {
                                        mpv::Event::PlaybackRestart => {
                                            info!("Playback has started successfully");
                                            break;
                                        }
                                        mpv::Event::EndFile(env_file_reason) => {
                                            match env_file_reason {
                                                Ok(_) => {
                                                    log::debug!("Start command has been sent!")
                                                }
                                                Err(err) => {
                                                    error!("{}", err);
                                                    break;
                                                }
                                            }
                                        }
                                        some_other_event => {
                                            debug!("Event: {:?}", some_other_event);
                                        }
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
