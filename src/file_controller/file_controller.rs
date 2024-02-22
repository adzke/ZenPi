use std::{fs, path::Path, env};

pub fn initialise_file_controller() {
    let binding = env::var_os("HOME").expect("Expect env var for user's home directory");
    let home_dir = binding.to_str().expect("Now to be a str");
    let application_name: &str = "zenpi";
    let application_state_data_path_str = format!("{}/.{}", home_dir, application_name);
    let application_state_data_path = Path::new(&application_state_data_path_str);
    log::debug!("Attempting to create this dir: {:?}", application_state_data_path_str);

    match fs::create_dir(application_state_data_path) {
        Ok(_) =>  {
            log::info!("Created folder at: {:?}", application_state_data_path)
        },
        Err(err) => match err.kind(){
            std::io::ErrorKind::AlreadyExists => (),
            err => panic!("Something SERIOUSLY bad has happened here family, error: {}", err),
        }
    }
    let audio_store_path_str = format!("{}/application_data/audio_files/", application_state_data_path_str);
    let audio_store_path = Path::new(&audio_store_path_str);
    log::debug!("Attempting to create this dir: {:?}", audio_store_path_str);

    match fs::create_dir_all(audio_store_path) {
        Ok(_) =>  {
            log::info!("Created folder at: {:?}", audio_store_path)
        },
        Err(err) => match err.kind(){
            std::io::ErrorKind::AlreadyExists => (),
            err => panic!("Something SERIOUSLY bad has happened here family, error: {}", err),
        }
    }
}

pub async fn main() {
    initialise_file_controller()
}