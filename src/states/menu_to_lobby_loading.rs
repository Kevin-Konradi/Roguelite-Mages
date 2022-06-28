use amethyst::assets::{AssetStorage, Completion, Handle, Loader, ProgressCounter};
use amethyst::renderer::{ImageFormat, Sprite, SpriteSheet, SpriteSheetFormat, Texture};
use amethyst::{EmptyState, EmptyTrans, GameData, SimpleState, SimpleTrans, State, StateData, StateEvent, Trans};
use amethyst::ecs::{Entity, World, WorldExt};
use amethyst::ecs::error::WrongGeneration;
use log::{error, info};
use crate::states::ingame_lobby::IngameLobby;

#[derive(Default)]
pub struct MenuToLobbyLoadingState {
    ui_root: Option<Entity>,
    progress_counter: ProgressCounter,
    player_sprite_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for MenuToLobbyLoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.player_sprite_handle = Some(self.load_player_spritesheet(world));
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
                if let Some(root) = self.ui_root {
                    match world.delete_entity(root) {
                        Ok(_) => {}
                        Err(_) => {
                            error!("failed to delete Ui Root Entity");
                        }
                    }
                }
                Trans::Switch(Box::new(IngameLobby {
                    player_texture_handle: self.player_sprite_handle.clone(),
                }))
            }
            Completion::Loading => {
                // LOADING ANIMATION
                Trans::None
            }
        }
    }
}

impl MenuToLobbyLoadingState {
    fn load_player_spritesheet(&mut self, world: &mut World) -> Handle<SpriteSheet> {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                "texture/player/player.png",
                ImageFormat::default(),
                &mut self.progress_counter,
                &world.read_resource::<AssetStorage<Texture>>(),
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "texture/player/player.ron",
            SpriteSheetFormat(texture_handle),
            &mut self.progress_counter,
            &sprite_sheet_store,
        )
    }
}