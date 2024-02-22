use serde::Deserialize;

use crate::file_controller::file_controller::Track;

pub mod api; // declare the module

#[derive(Debug, Deserialize, Clone)]

pub enum Command {
    Start,
    Stop
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub ipc_command: Command,
    pub track: Option<Track>,
}
