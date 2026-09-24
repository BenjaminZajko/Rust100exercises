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

mod data;
mod store;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};
use data::{Ticket, TicketDraft, TicketPatch};
use std::sync::{Arc, RwLock};
use store::{TicketId, TicketStore};

// zdielany stav servera ako Arc<RwLock<TicketStore>> len bez kanalov
type SharedStore = Arc<RwLock<TicketStore>>;

#[tokio::main]
async fn main() {
    let store: SharedStore = Arc::new(RwLock::new(TicketStore::new()));

    let app = Router::new()
        .route("/tickets", post(create_ticket))
        .route("/tickets/:id", get(get_ticket))
        .route("/tickets/:id", patch(patch_ticket))
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server bezi na http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

// POST /tickets
async fn create_ticket(
    State(store): State<SharedStore>,
    Json(draft): Json<TicketDraft>,
) -> (StatusCode, Json<TicketId>) {
    let id = store.write().unwrap().add_ticket(draft);
    (StatusCode::CREATED, Json(id))
}

// GET /tickets/:id
async fn get_ticket(
    State(store): State<SharedStore>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>, StatusCode> {
    let id = TicketId(id);
    let ticket_handle = store.read().unwrap().get(id).ok_or(StatusCode::NOT_FOUND)?;
    let ticket = ticket_handle.read().unwrap().clone();
    Ok(Json(ticket))
}

// PATCH /tickets/:id
async fn patch_ticket(
    State(store): State<SharedStore>,
    Path(id): Path<u64>,
    Json(patch): Json<TicketPatch>,
) -> Result<StatusCode, StatusCode> {
    let id = TicketId(id);
    let ticket_handle = store.read().unwrap().get(id).ok_or(StatusCode::NOT_FOUND)?;
    let mut ticket = ticket_handle.write().unwrap();

    if let Some(title) = patch.title {
        ticket.title = title;
    }
    if let Some(description) = patch.description {
        ticket.description = description;
    }
    if let Some(status) = patch.status {
        ticket.status = status;
    }

    Ok(StatusCode::OK)
}
