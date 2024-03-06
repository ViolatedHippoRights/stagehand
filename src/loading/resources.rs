use std::{borrow::Borrow, collections::HashMap, hash::Hash, rc::Rc, string::ToString};
use uuid::Uuid;

use super::{ResourceError, Ticket};

pub trait ResourceLoader<'a, R> {
    type Arguments: ?Sized;

    fn load(&'a self, args: &Self::Arguments) -> Result<R, ResourceLoadError>;
}

pub struct ResourceStorage<'a, K, R, L>
where
    K: Hash + Eq + ToString,
    L: 'a + ResourceLoader<'a, R>,
{
    loader: &'a L,
    store: Vec<Rc<R>>,
    map: HashMap<K, usize>,

    num_locks: u32,
    locked: bool,
    uuid: Uuid,
}

impl<'a, K, R, L> ResourceStorage<'a, K, R, L>
where
    K: Hash + Eq + ToString,
    L: ResourceLoader<'a, R>,
{
    pub fn new(loader: &'a L) -> Self {
        ResourceStorage {
            loader,
            locked: false,
            num_locks: 0,
            map: HashMap::new(),
            store: Vec::new(),
            uuid: uuid::Uuid::new_v4(),
        }
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
        self.num_locks += 1;
    }

    pub fn load<A>(&mut self, key: K, args: &A) -> Result<(), ResourceLoadError>
    where
        A: ?Sized,
        L: ResourceLoader<'a, R, Arguments = A>,
    {
        if self.map.contains_key(&key) {
            return Err(ResourceLoadError::AlreadyExists(key.to_string()));
        }

        self.store.push(Rc::new(self.loader.load(args)?));
        self.map.insert(key, self.store.len() - 1);

        Ok(())
    }

    pub fn get_by_key<KB>(&self, key: &KB) -> Result<Rc<R>, ResourceError>
    where
        KB: Hash + Eq + ToString + ?Sized,
        K: Borrow<KB>,
    {
        match self.map.get(key) {
            Some(r) => Ok(self.store[*r].clone()),
            None => Err(ResourceError::NotStored(key.to_string())),
        }
    }

    pub fn take_ticket<KB>(&self, key: &KB) -> Result<Ticket, ResourceError>
    where
        KB: Hash + Eq + ToString + ?Sized,
        K: Borrow<KB>,
    {
        match self.map.get(key) {
            Some(r) => Ok(Ticket {
                index: *r,
                storage_lock: self.num_locks,
                storage_uuid: self.uuid,
            }),
            None => Err(ResourceError::NotStored(key.to_string())),
        }
    }

    pub fn get_by_ticket(&self, ticket: Ticket) -> Result<Rc<R>, ResourceError> {
        if !self.locked {
            return Err(ResourceError::StorageUnlocked);
        }

        if self.uuid != ticket.storage_uuid {
            return Err(ResourceError::WrongStorage);
        }

        if self.num_locks != ticket.storage_lock {
            return Err(ResourceError::TicketOutdated);
        }

        Ok(self.store[ticket.index].clone())
    }

    pub fn get_by_ticket_unchecked(&self, ticket: Ticket) -> Rc<R> {
        self.store[ticket.index].clone()
    }
}

#[derive(Debug)]
pub enum ResourceLoadError {
    AlreadyExists(String),
    LoadFailure(String),
}
