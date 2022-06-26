use std::default::Default;
use amethyst::audio::AudioBundle;
use amethyst::core::TransformBundle;
use amethyst::{Application, GameDataBuilder};
use amethyst::input::{InputBundle, StringBindings};
use amethyst::renderer::{RenderFlat2D, RenderingBundle, RenderToWindow};
use amethyst::renderer::types::DefaultBackend;
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::utils::application_root_dir;
use crate::states::{MainMenu, MainMenuButtonSystemDesc};

mod states;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let prefab_dir = assets_dir.join("prefab");

    let display_config_path = config_dir.join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(MainMenuButtonSystemDesc::default(), "main_menu_event_handler", &[]);

    let mut game = Application::new(assets_dir, MainMenu::default(), game_data)?;
    game.run();

    Ok(())
}
