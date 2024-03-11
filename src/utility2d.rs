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

pub struct Initialize<I, S, C> {
    pub input: InputMap<I>,
    pub storage: S,
    pub content: C,
}

impl<I, S, C> Initialize<I, S, C> {
    pub fn new(input: InputMap<I>, storage: S, content: C) -> Self {
        Initialize {
            input,
            storage,
            content,
        }
    }
}

pub struct Update<I, C> {
    pub input: InputMap<I>,
    pub info: Vec<UpdateInfo>,
    pub content: C,
}

pub enum UpdateInfo {
    MusicStopped,
}

pub enum UpdateAction {
    PlayMusic(Ticket, i32, f32),
    PlaySound(Ticket, f32),
}
