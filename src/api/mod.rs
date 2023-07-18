use serde::Deserialize;

pub mod api; // declare the module



#[derive(Debug)]
#[derive(Deserialize)]
#[derive(Clone)]

pub enum Command {
    Start,
    Stop,
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct Message {
    pub ipc_command: Command
}