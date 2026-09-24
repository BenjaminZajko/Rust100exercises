use crate::store::TicketId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: String,
    pub description: String,
    pub status: Status,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TicketDraft {
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TicketPatch {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<Status>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}