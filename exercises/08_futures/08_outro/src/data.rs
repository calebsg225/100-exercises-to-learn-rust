//! src/data.rs

use crate::store::TicketId;
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}
