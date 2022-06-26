use std::collections::HashMap;
use amethyst::assets::Handle;
use amethyst::renderer::SpriteSheet;

pub struct IngameLobby {
    pub(crate) texture_handles: HashMap<String, Handle<SpriteSheet>>
}

