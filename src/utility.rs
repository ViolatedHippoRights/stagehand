use core::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::{input::InputMap, loading::Ticket};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum StorageType {
    Data,
    Font,
    Music,
    Sound,
    Texture,
}

impl fmt::Display for StorageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StorageType::Data => "Data",
                StorageType::Font => "Font",
                StorageType::Music => "Music",
                StorageType::Sound => "Sound",
                StorageType::Texture => "Texture",
            }
        )
    }
}

pub struct Initialize<I, S, C> {
    pub input: Rc<RefCell<InputMap<I>>>,
    pub storage: Rc<RefCell<S>>,
    pub content: Rc<RefCell<C>>,
}

impl<I, S, C> Initialize<I, S, C> {
    pub fn new(
        input: Rc<RefCell<InputMap<I>>>,
        storage: Rc<RefCell<S>>,
        content: Rc<RefCell<C>>,
    ) -> Self {
        Self {
            input,
            storage,
            content,
        }
    }
}

pub struct Update<I, C> {
    pub input: Rc<RefCell<InputMap<I>>>,
    pub info: Rc<RefCell<Vec<UpdateInfo>>>,
    pub content: Rc<RefCell<C>>,
}

impl<I, C> Update<I, C> {
    pub fn new(
        input: Rc<RefCell<InputMap<I>>>,
        info: Rc<RefCell<Vec<UpdateInfo>>>,
        content: Rc<RefCell<C>>,
    ) -> Self {
        Self {
            input,
            info,
            content,
        }
    }
}

pub enum UpdateInfo {
    MusicStopped,
}

pub enum UpdateInstruction {
    PlayMusic(Ticket, i32, f32),
    PlaySound(Ticket, f32),
}
