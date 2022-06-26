use std::collections::HashMap;
use amethyst::assets::{AssetStorage, Completion, Handle, Loader, ProgressCounter};
use amethyst::renderer::{ImageFormat, Sprite, SpriteSheet, SpriteSheetFormat, Texture};
use amethyst::{EmptyState, EmptyTrans, GameData, SimpleState, SimpleTrans, State, StateData, StateEvent, Trans};
use amethyst::ecs::{Entity, World, WorldExt};
use amethyst::renderer::rendy::hal::pass::AttachmentLoadOp::Load;
use log::{error, info};
use crate::MainMenu;
use crate::states::ingame_lobby::IngameLobby;

#[derive(Default)]
pub struct MenuToLobbyLoadingState {
    ui_root: Entity,
    progress_counter: ProgressCounter,
    texture_handles: HashMap<String, Handle<SpriteSheet>>
}

impl SimpleState for MenuToLobbyLoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.texture_handles.insert("player_sprites".to_string(), load_player_spritesheet(world, &self.progress_counter));
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &mut data.world;

        match self.progress_counter.complete() {
            Completion::Failed => {
                error!("Failed loading assets: {:?}", self.progress_counter.errors());
                Trans::Quit
            }
            Completion::Complete => {
                info!("Assets loaded, swapping state");
                // DELETE LOADING_ANIM ENTITIES
                world.delete_entity(self.ui_root);
                Trans::Switch(Box::new(IngameLobby {
                    texture_handles: *self.texture_handles
                }))
            }
            Completion::Loading => {
                // LOADING ANIMATION
                Trans::None
            }
        }
    }
}

fn load_player_spritesheet(world: &mut World, progress_counter: &ProgressCounter) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/player/player.png",
            ImageFormat::default(),
            &progress_counter,
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/player/player.ron",
        SpriteSheetFormat(texture_handle),
        &progress_counter,
        &sprite_sheet_store,
    )
}