use bevy::{prelude::*, window::PresentMode};
use bevy::window::WindowMode;

mod states;

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GameState {
    Splash,
    Menu,
    Game,
    Loading
}

fn main()  {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1920.,
            height: 1080.,
            title: "Roguelite Mages".to_string(),
            present_mode: PresentMode::Immediate,
            cursor_visible: true,
            cursor_locked: false,
            mode: WindowMode::Fullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_state(GameState::Game)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
