use serde::{Deserialize, Serialize};

pub const BUFFER_SIZE: usize = 1024;

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub alias: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    // boardcast message
    Broadcast,

    // close signal
    Close,
}
