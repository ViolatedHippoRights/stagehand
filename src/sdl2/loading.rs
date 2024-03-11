use sdl2::{
    image::LoadTexture,
    mixer::Music,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use std::{str, string::ToString};

use crate::{
    loading::{
        resources::{ResourceLoadError, ResourceLoader, ResourceStorage},
        ResourceError, Ticket, TicketManager,
    },
    utility2d::StorageType,
};

type TextureStorage<'a> = ResourceStorage<'a, String, Texture<'a>, TextureCreator<WindowContext>>;
type MusicStorage<'a> = ResourceStorage<'a, String, Music<'a>, ()>;

pub struct SDLStorage<'a> {
    pub textures: TextureStorage<'a>,
    pub music: MusicStorage<'a>,
}

impl<'a> SDLStorage<'a> {
    pub fn new(texture: &'a TextureCreator<WindowContext>) -> Self {
        SDLStorage {
            textures: TextureStorage::new(texture),
            music: MusicStorage::new(&()),
        }
    }
}

impl<'a> TicketManager<StorageType, StorageType, String, str> for SDLStorage<'a> {
    fn get_ticket_with_key(
        &self,
        storage_key: &StorageType,
        resource_key: &str,
    ) -> Result<Ticket, ResourceError> {
        match storage_key {
            StorageType::Texture => self.textures.take_ticket(resource_key),
            StorageType::Music => self.music.take_ticket(resource_key),
            _ => Err(ResourceError::UnknownStorage(storage_key.to_string())),
        }
    }
}

impl<'a, T> ResourceLoader<'a, Texture<'a>> for TextureCreator<T> {
    type Arguments = str;

    fn load(&'a self, args: &Self::Arguments) -> Result<Texture<'a>, ResourceLoadError> {
        let result = self.load_texture(args);
        match result {
            Ok(t) => Ok(t),
            Err(e) => Err(ResourceLoadError::LoadFailure(e)),
        }
    }
}

impl<'a> ResourceLoader<'a, Music<'a>> for () {
    type Arguments = str;

    fn load(&'a self, args: &Self::Arguments) -> Result<Music<'a>, ResourceLoadError> {
        match sdl2::mixer::Music::from_file(args) {
            Ok(m) => Ok(m),
            Err(e) => Err(ResourceLoadError::LoadFailure(e)),
        }
    }
}
