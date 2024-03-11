use log::error;
use std::{borrow::Borrow, hash::Hash, string::ToString};
use uuid::Uuid;

pub mod resources;

#[derive(Clone, Copy)]
pub struct Ticket {
    index: usize,
    storage_uuid: Uuid,
    storage_lock: u32,
}

pub trait TicketManager<SK, SKB, K, KB>
where
    SKB: Hash + Eq + ToString + ?Sized,
    KB: Hash + Eq + ToString + ?Sized,
    K: Hash + Eq + ToString + Borrow<KB>,
{
    fn get_ticket_with_key(
        &self,
        storage_key: &SKB,
        resource_key: &KB,
    ) -> Result<Ticket, ResourceError>;
}

#[derive(Debug)]
pub enum ResourceError {
    NotStored(String),
    StorageUnlocked,
    TicketOutdated,
    UnknownStorage(String),
    WrongStorage,
}

impl ResourceError {
    pub fn log_failure(e: ResourceError) {
        match e
        {

            ResourceError::NotStored(err) => error!("Missing resource with key: {}", err),
            ResourceError::StorageUnlocked => error!("Attempting to access unlocked storage."),
            ResourceError::TicketOutdated => error!("Attempting to access storage with outdated ticket."),
            ResourceError::UnknownStorage(err) => error!("Attempting to access storage with unknown key: {}", err),
            ResourceError::WrongStorage => error!("Attempting to access storage with ticket corresponding to the wrong resource type.")

        }
    }
}
