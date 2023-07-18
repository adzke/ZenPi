
mod api;
mod media_controller; 

use std::sync::{mpsc::channel, Arc, Mutex};
use crate::api::Message;



#[tokio::main]
async fn main() {
    println!("Starting worker threads");
let (tx, rx) = channel::<Message>();

let tx = Arc::new(Mutex::new(tx));
let rx = Arc::new(Mutex::new(rx));





    let t1: tokio::task::JoinHandle<()> = tokio::spawn( async {
        let _ = api::api::main(tx).await;
    });

    let t2: tokio::task::JoinHandle<()> = tokio::spawn(async {
        media_controller::media_controller::main(rx);
    });

    let _ = tokio::join!(t1, t2);
}

