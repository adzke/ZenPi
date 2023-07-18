
use std::sync::{mpsc::Sender, Arc, Mutex};

use poem::{get, handler, listener::TcpListener, web::{Path, Data, Json}, Route, Server, EndpointExt, post};

use super::Message;

#[handler]
async fn hello(Path(name): Path<String>, data: Data<&Arc<Mutex<Sender<Message>>>>) -> String {

    let message = Message {
        ipc_command: format!("hello: {}", name).to_string()
    };

    data.lock().unwrap().send(message).expect("failed to send");
    format!("hello: {}", name)
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
    .at("/hello/:name", get(hello).data(tx.clone()))
    .at("/time_remaining/", post(time_remaining).data(tx.clone()));
    let _ = Server::new(TcpListener::bind("0.0.0.0:4000"))
        .run(app).await?;
    Ok(())
}

