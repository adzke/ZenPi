use std::{
    env,
    ffi::OsString,
    fs::{self, DirEntry, File},
    path::Path,
};

pub struct FileController {
    binding: OsString,
    home_dir: String,
    application_name: String,
    application_state_data_path_str: String,
    audio_store_path_str: String,
}

impl FileController {
    pub fn new() -> Self {
        let binding = env::var_os("HOME").expect("Expect env var for user's home directory");
        let home_dir = binding
            .clone()
            .to_str()
            .expect("Now to be a str")
            .to_string();
        let application_name = "zenpi".to_string();
        let application_state_data_path_str = format!("{}/.{}", home_dir, application_name);
        let audio_store_path_str = format!(
            "{}/application_data/audio_files/",
            application_state_data_path_str
        );
        Self {
            binding: binding.clone(),
            home_dir: binding
                .clone()
                .to_str()
                .expect("Now to be a str")
                .to_string(),
            application_name: "zenpi".to_string(),
            application_state_data_path_str,
            audio_store_path_str,
        }
    }

    pub fn initialise_file_controller(self) -> Self {
        log::debug!(
            "Attempting to create this dir: {:?}",
            self.application_state_data_path_str.clone()
        );

        match fs::create_dir(self.application_state_data_path_str.clone()) {
            Ok(_) => {
                log::info!(
                    "Created folder at: {:?}",
                    self.application_state_data_path_str
                )
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::AlreadyExists => (),
                err => panic!(
                    "Something SERIOUSLY bad has happened here family, error: {}",
                    err
                ),
            },
        }

        let audio_store_path = Path::new(&self.audio_store_path_str);
        log::debug!(
            "Attempting to create this dir: {:?}",
            self.audio_store_path_str
        );

        match fs::create_dir_all(audio_store_path) {
            Ok(_) => {
                log::info!("Created folder at: {:?}", audio_store_path);
                self
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::AlreadyExists => self,
                err => panic!(
                    "Something SERIOUSLY bad has happened here family, error: {}",
                    err
                ),
            },
        }
    }

    pub fn list_files(&self) -> Vec<String> {
        let files_directory = fs::read_dir(self.audio_store_path_str.clone()).expect("Should return directory");
        let mut file_vec = Vec::<String>::new();
        for file in files_directory {
            match file {
                Ok(file) => file_vec.push(file.file_name().to_str().expect("Should now be a str").to_string()),
                Err(_) => continue
            }
        }
        file_vec
    }
}

pub async fn main() {
}
