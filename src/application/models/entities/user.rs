use std::fmt::{self, Debug, Display};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Activity {
    Connected,
    Disconnected,
    Muted,
    Unmuted,
    OpenedStream,
    ClosedStream,
    OpenedCamera,
    ClosedCamera,
    Moved
}

impl Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}
