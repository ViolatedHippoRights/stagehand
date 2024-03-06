use core::fmt;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum StorageType {
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
                StorageType::Music => "Music",
                StorageType::Sound => "Sound",
                StorageType::Texture => "Texture",
            }
        )
    }
}
