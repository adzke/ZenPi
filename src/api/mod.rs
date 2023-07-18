pub mod api; // declare the module

#[derive(Debug)]
pub struct Message {
    pub ipc_command: String
}