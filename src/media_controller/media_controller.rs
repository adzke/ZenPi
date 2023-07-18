use mpv::{Event, MpvHandler};
use std::sync::{Arc, Mutex, mpsc::Receiver};

use crate::api::Message;

//
// API
//

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

pub fn load_file(player: Arc<Mutex<MpvHandler>>) -> () {
    log::info!("Loading file into MPV Player");

    let command_array = ["loadfile", "/home/ad/Downloads/delta.m4a"];

    player
        .lock()
        .unwrap()
        .command(&command_array)
        .expect("Failed to execute command");
}

fn human_time_remaining(seconds: &f64) -> String {
    let time = seconds.round() as u64;
    let days = time / 86400;
    let hours = (time % 86400) / 3600;
    let minutes = ((time % 86400) % 3600) / 60;
    let seconds = ((time % 86400) % 3600) % 60;

    return format!(
        "days: {}, hours: {}, minutes: {}, seconds: {}, remaing.",
        days, hours, minutes, seconds
    );
}


pub fn main(rx: Arc<Mutex<Receiver<Message>>>) {
    configure_logger();

    log::info!("Starting ZENPLAYER by AD");
    let player = configure_player();
    let player = Arc::new(Mutex::new(player));

    log::info!("Loading file into MPV Player");

    load_file(player.clone());


    loop {

        let mut player_lock = player.lock().unwrap();

        if let Ok(message) = rx.clone().lock().unwrap().try_recv(){
            println!("{:?}", message.ipc_command);
            
            if let Ok(result) = player_lock.command(&[&format!("{}", message.ipc_command)]){
                println!("{:?}", result)
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
