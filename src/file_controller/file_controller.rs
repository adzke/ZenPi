use serde::{Deserialize, Serialize};
use std::{
    env, ffi::OsString, fs, path::{Path, PathBuf}, time::SystemTime
};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Track {
    track_id: String,
    track_name: String,
    track_path: PathBuf,
    track_created_time: SystemTime,
}

pub struct FileController {
    _binding: OsString,
    _home_dir: String,
    _application_name: String,
    application_state_data_path_str: String,
    audio_store_path_str: String,
    tracks: Vec<Track>,
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
            _binding: binding.clone(),
            _home_dir: binding
                .clone()
                .to_str()
                .expect("Now to be a str")
                .to_string(),
            _application_name: "zenpi".to_string(),
            application_state_data_path_str,
            audio_store_path_str,
            tracks: Vec::<Track>::new(),
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

    pub fn list_files(&mut self) -> Vec<Track> {
        let files_directory =
            fs::read_dir(self.audio_store_path_str.clone()).expect("Should return directory");
        for file in files_directory {
            match file {
                Ok(file) => match self.tracks.iter().find(|t| t.track_path == file.path()) {
                    Some(_) => continue,
                    None => {
                        let new_track = Track {
                            track_id: Uuid::new_v4().to_string(),
                            track_name: file
                                .file_name()
                                .into_string()
                                .expect("OsString to be converted to Str"),
                            track_created_time: file
                                .metadata()
                                .expect("Metadata should be present")
                                .created()
                                .expect("Expect a created time from metadata"),
                            track_path: file.path(),
                        };
                        self.tracks.push(new_track);
                    }
                },

                Err(_) => continue,
            }
        }
        self.tracks.clone()
    }

    pub fn find_track(&mut self, track_id: &str) -> Result<Track, String> {
        match self.tracks.iter().find(|x| x.track_id == track_id) {
            Some(track) => Ok(track.clone()),
            None => Err(format!("Failed to find track by id: {}", track_id))
        }
    }
}

pub async fn main() {}
