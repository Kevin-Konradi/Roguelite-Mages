use std::collections::HashMap;
use amethyst::assets::Handle;
use amethyst::renderer::{Camera, SpriteSheet};
use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans};
use amethyst::core::Transform;
use amethyst::ecs::{Builder, World, WorldExt};
use amethyst::input::is_key_down;
use amethyst::window::ScreenDimensions;
use amethyst::winit::VirtualKeyCode;
use crate::states::pause::PauseState;

pub struct IngameLobby {
    pub player_texture_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for IngameLobby {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world)
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(window_event) => {
                if is_key_down(&window_event, VirtualKeyCode::Escape) {
                    Trans::Push(Box::new(PauseState::default()))
                } else {
                    Trans::None
                }
            }
            _ => { Trans::None }
        }
    }
}

fn initialise_camera(world: &mut World) {
    let (height, width) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.height(), dim.width())
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(width / 2.0, height / 2.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}