// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{SyncSender, Sender, Receiver};

pub mod data;
pub mod store;

#[derive(Debug)]

#[derive(Clone)]
pub struct Error;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, Error> {
        let (response_channel, response_receiver) = std::sync::mpsc::channel();
        self.sender
            .send(Command::Insert {
                draft,
                response_channel,
            })
            .map_err(|_| Error)?;
        response_receiver.recv().map_err(|_| Error)
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, Error> {
        let (response_channel, response_receiver) = std::sync::mpsc::channel();
        self.sender
            .send(Command::Get {
                id,
                response_channel,
            })
            .map_err(|_| Error)?;
        response_receiver.recv().map_err(|_| Error)
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
