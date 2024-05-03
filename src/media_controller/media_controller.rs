use crate::{
    api::{Command, Message},
    file_controller::file_controller::FileController,
};
use core::panic;
use log;
use std::{process::Child, sync::Arc};
use tokio::sync::{mpsc::Receiver, Mutex};

pub async fn main(
    rx: Arc<Mutex<Receiver<Message>>>,
    file_controller: Arc<Mutex<FileController>>,
) {
    let mut child_proc: Option<Child> = None;
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
                                    }
                                }
                            }
                        }
                    };

                    log::debug!("Track path has been defined");

                    child_proc = Some(
                        std::process::Command::new("mpv")
                            .arg(track_path.to_str().expect("expect str"))
                            .spawn()
                            .unwrap(),
                    );
                }

                Command::Stop => {
                    if let Some(ref mut child) = child_proc {
                        child.kill().unwrap();
                        log::info!("ZenPi has stopped playing.");
                    }
                }
            }
        }
    }
}
