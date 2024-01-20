use std::sync::Arc;

use poem::{
    get, handler,
    listener::TcpListener,
    post,
    web::{Data, Json, Path},
    EndpointExt, Route, Server,
};
use tokio::sync::{Mutex, mpsc::Sender};

use crate::api::Command;

use super::Message;

#[handler]
async fn start(data: Data<&Arc<Mutex<Sender<Message>>>>) -> String {
    let message = Message {
        ipc_command: Command::Start,
    };

    data.lock().await.send(message).await.expect("failed to send");
    format!("Start command sent")
}

#[handler]
async fn stop(data: Data<&Arc<Mutex<Sender<Message>>>>) -> String {
    let message = Message {
        ipc_command: Command::Stop,
    };

    data.lock().await.send(message).await.expect("failed to send");
    format!("Stop command sent")
}

#[handler]
async fn time_remaining(data: Data<&Arc<Mutex<Sender<Message>>>>) -> String {
    let message = Message {
        ipc_command: Command::TimeRemaining,
    };

    data.lock().await.send(message).await.expect("failed to send");
    format!("Stop command sent")
}

pub async fn main(tx: Arc<Mutex<Sender<Message>>>) -> Result<(), std::io::Error> {
    println!("Starting server");
    let app = Route::new()
        .at("/stop/", post(stop).data(tx.clone()))
        .at("/start/", post(start).data(tx.clone()))
        .at("/time_remaining/", post(time_remaining).data(tx.clone()));
    let _ = Server::new(TcpListener::bind("0.0.0.0:4000"))
        .run(app)
        .await?;
    Ok(())
}
