use serde::Deserialize;

pub mod api; // declare the module

#[derive(Debug, Deserialize, Clone)]

pub enum Command {
    Start,
    Stop,
    TimeRemaining,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub ipc_command: Command,
}
