use std::fmt::{self, Debug, Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Signature {
    pub emojis: String,
    pub user_id: u64,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum Activity {
    Connected,
    Disconnected,
    Muted,
    Unmuted,
    OpenedStream,
    ClosedStream,
    OpenedCamera,
    ClosedCamera,
    Moved,
}

impl Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}
