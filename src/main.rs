mod api;
mod media_controller;

use tokio::sync::{Mutex, mpsc::channel};

use crate::{api::Message, media_controller::media_controller::configure_player};
use std::{sync::{Arc}, ops::{Deref, DerefMut}};



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
    env_logger::init();
}

#[tokio::main]
async fn main() {


    let created_player = configure_player();
    let send_player = UnsafeSend(created_player);
    let player = Arc::new(Mutex::new(send_player));
    let player_clone = Arc::clone(&player);
    configure_logger();
    log::info!("Starting ZENPLAYER by AD");
    let (tx, rx) = channel::<Message>(512);

    let tx = Arc::new(Mutex::new(tx));
    let rx = Arc::new(Mutex::new(rx));

    let t1: tokio::task::JoinHandle<()> = tokio::spawn(async {
        
        let _ = api::api::main(tx).await;
    });

    let t2: tokio::task::JoinHandle<()> = tokio::spawn(async {
        let _ = media_controller::media_controller::main(rx, player_clone).await;
    });

    let _ = tokio::join!(t1, t2);
}
