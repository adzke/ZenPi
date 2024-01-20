use std::sync::Arc;
use log::info;
use super::Message;
use crate::api::Command;
use poem::{
    get, handler,
    listener::TcpListener,
    post,
    web::{Data, Html},
    EndpointExt, Route, Server,
};
use tokio::sync::{mpsc::Sender, Mutex};

#[handler]
async fn start(data: Data<&Arc<Mutex<Sender<Message>>>>) -> String {
    let message = Message {
        ipc_command: Command::Start,
    };

    data.lock()
        .await
        .send(message)
        .await
        .expect("failed to send");
    format!("Start command sent")
}

#[handler]
async fn stop(data: Data<&Arc<Mutex<Sender<Message>>>>) -> String {
    let message = Message {
        ipc_command: Command::Stop,
    };

    data.lock()
        .await
        .send(message)
        .await
        .expect("failed to send");
    format!("Stop command sent")
}

#[handler]
async fn time_remaining(data: Data<&Arc<Mutex<Sender<Message>>>>) -> String {
    let message = Message {
        ipc_command: Command::TimeRemaining,
    };

    data.lock()
        .await
        .send(message)
        .await
        .expect("failed to send");
    format!("Stop command sent")
}

#[handler]
async fn html_controller() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Control Server</title>
        </head>
        <body>

        <h1>Control Server</h1>

        <button onclick="sendCommand('/start/')">Start</button>
        <button onclick="sendCommand('/stop/')">Stop</button>
        <button onclick="sendCommand('/time_remaining/')">Time Remaining</button>

        <script>
            async function sendCommand(endpoint) {
                try {
                    const response = await fetch(`http://192.168.0.112:4000${endpoint}`, {
                        method: 'POST',
                    });

                    if (response.ok) {
                        const result = await response.text();
                        alert(result);
                    } else {
                        alert(`Failed to send command: ${response.statusText}`);
                    }
                } catch (error) {
                    alert(`Error: ${error.message}`);
                }
            }
        </script>

        </body>
        </html>
    "#,
    )
}

pub async fn main(tx: Arc<Mutex<Sender<Message>>>) -> Result<(), std::io::Error> {
    info!("Starting server");
    let app = Route::new()
        .at("/stop/", post(stop).data(tx.clone()))
        .at("/start/", post(start).data(tx.clone()))
        .at("/time_remaining/", post(time_remaining).data(tx.clone()))
        .at("/", get(html_controller));
    let _ = Server::new(TcpListener::bind("0.0.0.0:4000"))
        .run(app)
        .await?;
    Ok(())
}
