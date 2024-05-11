mod api;
mod file_controller;
mod media_controller;
use crate::{api::Message, file_controller::file_controller::FileController};
use log;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tokio::sync::{mpsc::channel, Mutex};

pub struct UnsafeSend<T>(T);
unsafe impl<T> Send for UnsafeSend<T> {}

impl<T> Deref for UnsafeSend<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for UnsafeSend<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn configure_logger() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
}

#[tokio::main]
async fn main() {
    configure_logger();
    std::process::Command::new("bluetoothctl").arg("connect").arg("88:C6:26:5A:3F:BF").spawn().unwrap();
    let file_controller = FileController::new()
        .initialise_file_controller()
        .initialise_files();
    let file_controller_protected = Arc::new(Mutex::new(file_controller));

    let file_controller_protected_api = Arc::clone(&file_controller_protected);
    let file_controller_protected_media_controller = Arc::clone(&file_controller_protected);

    log::info!("Starting ZenPi by AD");
    let (tx, rx) = channel::<Message>(512);
    let tx = Arc::new(Mutex::new(tx));
    let rx = Arc::new(Mutex::new(rx));
    let t1: tokio::task::JoinHandle<()> = tokio::spawn(async {
        let _ = api::api::main(tx, file_controller_protected_api).await;
    });
    let t2: tokio::task::JoinHandle<()> = tokio::spawn(async {
        let _ = media_controller::media_controller::main(
            rx,
            file_controller_protected_media_controller,
        )
        .await;
    });
    let t3: tokio::task::JoinHandle<()> = tokio::spawn(async {
        let _ = file_controller::file_controller::main().await;
    });
    let _ = tokio::join!(t1, t2, t3);
}
