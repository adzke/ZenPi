use serde::Deserialize;

pub mod api; // declare the module

#[derive(Debug)]
#[derive(Deserialize)]

pub struct Message {
    pub ipc_command: String
}