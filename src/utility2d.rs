use core::fmt;

use crate::{input::InputMap, loading::Ticket};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum StorageType {
    Data,
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
                StorageType::Music => "Music",
                StorageType::Sound => "Sound",
                StorageType::Texture => "Texture",
            }
        )
    }
}

pub struct Initialize<I, C> {
    pub input: InputMap<I>,
    pub content: C,
}

impl<I, C> Initialize<I, C> {
    pub fn new(input: InputMap<I>, content: C) -> Self {
        Initialize { input, content }
    }
}

pub struct Update<C> {
    pub input: InputMap<C>,
    pub info: Vec<UpdateInfo>,
}

pub enum UpdateInfo {
    MusicStopped,
}

pub enum UpdateAction {
    PlayMusic(Ticket, i32, f32),
    PlaySound(Ticket, f32),
}
