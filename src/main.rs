mod api;
mod media_controller;

use crate::api::Message;
use std::sync::{mpsc::channel, Arc, Mutex};

fn configure_logger() {
    env_logger::init();
}

#[tokio::main]
async fn main() {
    configure_logger();
    log::info!("Starting ZENPLAYER by AD");
    let (tx, rx) = channel::<Message>();

    let tx = Arc::new(Mutex::new(tx));
    let rx = Arc::new(Mutex::new(rx));

    let t1: tokio::task::JoinHandle<()> = tokio::spawn(async {
        let _ = api::api::main(tx).await;
    });

    let t2: tokio::task::JoinHandle<()> = tokio::spawn(async {
        media_controller::media_controller::main(rx);
    });

    let _ = tokio::join!(t1, t2);
}
