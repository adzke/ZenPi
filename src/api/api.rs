
use std::sync::{mpsc::Sender, Arc, Mutex};

use poem::{get, handler, listener::TcpListener, web::{Path, Data, Json}, Route, Server, EndpointExt, post};

use crate::api::Command;

use super::Message;


#[handler]
async fn start(data: Data<&Arc<Mutex<Sender<Message>>>>) -> String {

    let message = Message {
        ipc_command: Command::Start
    };

    data.lock().unwrap().send(message).expect("failed to send");
    format!("Start command sent")
}

#[handler]
async fn stop(data: Data<&Arc<Mutex<Sender<Message>>>>) -> String {

    let message = Message {
        ipc_command: Command::Stop
    };

    data.lock().unwrap().send(message).expect("failed to send");
    format!("Stop command sent")
}



#[handler]
async fn time_remaining(body: Json<Message>, data: Data<&Arc<Mutex<Sender<Message>>>>) -> String {

    let message = Message {
        ipc_command: body.ipc_command.clone()
    };

    data.lock().unwrap().send(message).expect("failed to send");
    format!("Command has been sent")
}

pub async fn main(tx: Arc<Mutex<Sender<Message>>>) -> Result<(), std::io::Error>{
    println!("Starting server");
    let app = Route::new()
    .at("/time_remaining/", post(time_remaining).data(tx.clone()))
    .at("/stop/", post(stop).data(tx.clone()))
    .at("/start/", post(start).data(tx.clone()));
    let _ = Server::new(TcpListener::bind("0.0.0.0:4000"))
        .run(app).await?;
    Ok(())
}

