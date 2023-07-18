use mpv::{Event, MpvHandler};
use std::sync::{mpsc::Receiver, Arc, Mutex, MutexGuard};

use crate::api::Message;

fn configure_logger() {
    env_logger::init();
}

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

pub fn main(rx: Arc<Mutex<Receiver<Message>>>) {
    let player = configure_player();
    let player = Arc::new(Mutex::new(player));

    log::info!("Loading file into MPV Player");

    loop {
        let mut player_lock = player.lock().unwrap();

        if let Ok(message) = rx.clone().lock().unwrap().try_recv() {
            println!("{:?}", message.ipc_command);
            match message.ipc_command {
                crate::api::Command::Start => {
                    let command_array = ["loadfile", "/home/ad/Downloads/delta.m4a"];
                    let _ = &player_lock
                        .command(&command_array)
                        .expect("Failed to execute command");
                }
                crate::api::Command::Stop => {
                    let command_array = ["stop"];
                    let _ = &player_lock
                        .command(&command_array)
                        .expect("Failed to execute command");
                }
            }
        }

        if let Some(event) = player_lock.wait_event(0.0) {
            println!("{:?}", event);
            match event {
                Event::EndFile(_) => {
                    println!("all done :D")
                }
                _ => (),
            }
        }
    }
}
