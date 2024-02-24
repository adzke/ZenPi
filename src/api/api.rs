use super::Message;
use crate::{
    api::Command,
    file_controller::file_controller::{FileController, Track},
};
use log::info;
use poem::{
    get, handler,
    listener::TcpListener,
    post,
    web::{Data, Html, Json},
    EndpointExt, Route, Server,
};
use std::sync::Arc;
use tokio::sync::{mpsc::Sender, Mutex};

#[handler]
async fn start(data: Data<&(Arc<Mutex<Sender<Message>>>, Arc<Mutex<FileController>>)>) -> String {
    match data
        .1
        .lock()
        .await
        .find_track("3a9788ca-7a5c-460e-b85e-357b14a129b7")
    {
        Ok(track) => {
            let message = Message {
                ipc_command: Command::Start,
                track: Some(track),
            };
            data.0
                 .0
                .lock()
                .await
                .send(message)
                .await
                .expect("failed to send");
            format!("Start command sent")
        }
        Err(err) => format!("{}", err),
    }
}

#[handler]
async fn stop(data: Data<&(Arc<Mutex<Sender<Message>>>, Arc<Mutex<FileController>>)>) -> String {
    let message = Message {
        ipc_command: Command::Stop,
        track: None,
    };

    data.0
         .0
        .lock()
        .await
        .send(message)
        .await
        .expect("failed to send");
    format!("Stop command sent")
}

#[handler]
async fn list_tracks(
    data: Data<&(Arc<Mutex<Sender<Message>>>, Arc<Mutex<FileController>>)>,
) -> Json<Vec<Track>> {
    let files = data.0 .1.lock().await.list_files();
    Json(files)
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
        <button onclick="getRequest('/list/')">List</button>

        

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

            async function getRequest(endpoint) {
                try {
                    const response = await fetch(`http://192.168.0.112:4000${endpoint}`, {
                        method: 'Get',
                    });

                    if (response.ok) {
                        const result = await response.json();
                        console.log(result)
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

pub async fn main(
    tx: Arc<Mutex<Sender<Message>>>,
    file_controller: Arc<Mutex<FileController>>,
) -> Result<(), std::io::Error> {
    let data = (tx.clone(), file_controller.clone());
    let default_address = "0.0.0.0:4000";
    info!("Starting server");
    let app = Route::new()
        .at("/stop/", post(stop).data(data.clone()))
        .at("/start/", post(start).data(data.clone()))
        .at("/list/", get(list_tracks).data(data.clone()))
        .at("/", get(html_controller));
    info!("Started server, running at {}:", default_address);
    let _ = Server::new(TcpListener::bind(default_address))
        .run(app)
        .await?;
    Ok(())
}
