// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

pub mod data;
pub mod store;

pub async fn ticket_store(listener: TcpListener, store: TicketStore) {
    let store = Arc::new(RwLock::new(TicketStore::new()));
    // set up store
    // run listeners
    // determine message?
}

async fn create_ticket(listener: TcpListener) {}
async fn retrieve_ticket(listener: TcpListener) {}
async fn patch_ticket(listener: TcpListener) {}

#[cfg(test)]
mod tests {
    use super::*;

    use std::net::SocketAddr;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        (listener, address)
    }

    #[tokio::test]
    async fn works() {}
}
