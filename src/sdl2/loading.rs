use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use std::str;

use crate::loading::{
    resources::{ResourceLoadError, ResourceLoader, ResourceStorage},
    utility2d::StorageType,
    ResourceError, Ticket, TicketManager,
};

type TextureStorage<'a> = ResourceStorage<'a, String, Texture<'a>, TextureCreator<WindowContext>>;

pub struct SDLStorage<'a> {
    pub textures: TextureStorage<'a>,
}

impl<'a> SDLStorage<'a> {
    pub fn new(creator: &'a TextureCreator<WindowContext>) -> Self {
        SDLStorage {
            textures: TextureStorage::new(creator),
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
