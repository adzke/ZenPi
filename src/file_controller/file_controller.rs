use log;
use serde::{Deserialize, Serialize};
use std::{
    env,
    ffi::OsString,
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
    time::SystemTime,
};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Track {
    pub track_id: String,
    pub track_name: String,
    pub track_path: PathBuf,
    pub track_created_time: SystemTime,
}

pub struct FileController {
    _binding: OsString,
    _home_dir: String,
    _application_name: String,
    manifest_path_str: String,
    application_state_data_path_str: String,
    pub audio_store_path_str: String,
    tracks: Vec<Track>,
    pub default_track: Option<Track>,
}

impl FileController {
    pub fn new() -> Self {
        log::debug!("Creating new instance of FileController");
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
        let manifest_file_path = format!(
            "{}/application_data/.manifest.json",
            application_state_data_path_str.clone()
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
            manifest_path_str: manifest_file_path,
            tracks: Vec::<Track>::new(),
            default_track: None,
        }
    }

    pub fn initialise_files(mut self) -> Self {
        log::info!("initialising files from file controller");
        let read_file: File = File::open(self.manifest_path_str.clone()).unwrap();
        log::debug!("Opened file: {:?}", read_file);
        let reader = BufReader::new(read_file);
        log::debug!("Reader {:?}", reader);
        let files_in_audio_folder: Vec<Track> = self.list_files();
        log::debug!("Found files in audio folder {:?}", files_in_audio_folder);
        let tracks: Result<Vec<Track>, serde_json::Error> = serde_json::from_reader(reader);
        log::debug!("Manifest file: {:?}", tracks);
        match tracks {
            Ok(tracks) => {
                log::debug!("Found Vec of Tracks");
                let mut tracks_to_write = Vec::<Track>::new();
                for track in tracks {
                    match files_in_audio_folder
                        .iter()
                        .find(|x| x.track_path != track.track_path)
                    {
                        Some(track) => {
                            log::debug!("{:?}", track);
                            log::debug!("{:?}", files_in_audio_folder[0].track_path);
                            tracks_to_write.push(track.clone())
                        }
                        None => continue,
                    }
                    log::debug!("{:?}", track);
                }
                {
                    let write_file = File::options()
                        .write(true)
                        .open(self.manifest_path_str.clone())
                        .unwrap();
                    log::debug!("{:?}", write_file);
                    let writer = BufWriter::new(write_file);
                    serde_json::to_writer(writer, &tracks_to_write).unwrap();
                }
                log::debug!("Found tracks to write {:?}", tracks_to_write.len());
                self.default_track = Some(self.tracks[0].clone());
            }
            Err(err) => {
                log::error!("{err}");
                {
                    let write_file = File::options()
                        .write(true)
                        .open(self.manifest_path_str.clone())
                        .unwrap();
                    log::debug!("{:?}", write_file);
                    let writer = BufWriter::new(write_file);
                    serde_json::to_writer(writer, &files_in_audio_folder).unwrap();
                }
            }
        }
        self
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
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::AlreadyExists => (),
                err => panic!(
                    "Something SERIOUSLY bad has happened here family, error: {}",
                    err
                ),
            },
        }
        match fs::File::open(self.manifest_path_str.clone()) {
            Ok(_) => {
                log::debug!(
                    "File already exists, moving along! {:?}",
                    self.manifest_path_str
                );
                return self;
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    match fs::File::create(self.manifest_path_str.clone()) {
                        Ok(_) => (),
                        Err(err) => match err.kind() {
                            err => panic!(
                                "Something SERIOUSLY bad has happened here family, error: {}",
                                err
                            ),
                        },
                    }
                    log::debug!("File already exists, not operation needed");
                    return self;
                }
                err => panic!(
                    "Something SERIOUSLY bad has happened here family, error: {}",
                    err
                ),
            },
        }
    }

    pub fn list_files(&mut self) -> Vec<Track> {
        log::debug!("Listing files on disk.");
        let supported_file_extentions = vec!["mp3", "m4a", "wav"];
        let files_directory =
            fs::read_dir(self.audio_store_path_str.clone()).expect("Should return directory");
        for file in files_directory {
            match file {
                Ok(file) => match self.tracks.iter().find(|t| t.track_path == file.path()) {
                    Some(_) => continue,
                    None => {
                        let file_extention = file
                            .path()
                            .extension()
                            .expect("expect path.")
                            .to_str()
                            .unwrap()
                            .to_string();
                        match supported_file_extentions
                            .iter()
                            .find(|s| s == &&file_extention)
                        {
                            Some(_) => {
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
                            None => continue,
                        }
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
            None => Err(format!("Failed to find track by id: {}", track_id)),
        }
    }
}

pub async fn main() {}
