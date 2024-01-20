use std::sync::Arc;

use mpv::{MpvHandler, Event};
use tokio::sync::{Mutex, mpsc::Receiver};

use crate::{api::{Message, Command}, UnsafeSend};

pub fn configure_player() -> MpvHandler {
    log::info!("Configuring MPV Player");
    let mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");
    // mpv_builder
    //     .set_option("sid", "no")
    //     .expect("Failed to set option 'sid' to 'no'");
    return mpv_builder.build().expect("failed to build");
}

// fn human_time_remaining(seconds: &f64) -> String {
//     let time = seconds.round() as u64;
//     let days = time / 86400;
//     let hours = (time % 86400) / 3600;
//     let minutes = ((time % 86400) % 3600) / 60;
//     let seconds = ((time % 86400) % 3600) % 60;

//     return format!(
//         "days: {}, hours: {}, minutes: {}, seconds: {}, remaing.",
//         days, hours, minutes, seconds
//     );
// }

pub async fn main(rx: Arc<Mutex<Receiver<Message>>>, player: Arc<Mutex<UnsafeSend<MpvHandler>>> ) {

    log::info!("Loading file into MPV Player");

    loop {

        if let Ok(message) = rx.clone().lock().await.try_recv() {
            println!("{:?}", message.ipc_command);
            match message.ipc_command {
                Command::Start => {
                    let command_array = ["loadfile", "/home/ad/Downloads/delta.m4a"];
                    let _ = player.lock().await
                        .command(&command_array)
                        .expect("Failed to execute command");
                }
                Command::Stop => {
                    let command_array = ["stop"];
                    let _ = player.lock().await
                        .command(&command_array)
                        .expect("Failed to execute command");
                },
                Command::TimeRemaining => {
                    let command_array = ["time-remaining"];
                    let _ = player.lock().await
                        .command(&command_array)
                        .expect("Failed to execute command");
                }
            }
        }

        // if let Some(event) = player.lock().await.wait_event(1.0) {
        //     println!("{:?}", event);
        //     match event {
        //         Event::EndFile(_) => {
        //             println!("all done :D")
        //         }
        //         _ => (),
        //     }
        // }
    }
}
