use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans};
use amethyst::ecs::{Entity, World};
use amethyst::input::{is_key_down, VirtualKeyCode};
use amethyst::ui::UiCreator;

#[derive(Default)]
pub struct PauseState {
    ui_root: Option<Entity>,
}

impl SimpleState for PauseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.initialise_ui(world);
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(window_event) => {
               if is_key_down(&window_event, VirtualKeyCode::Escape) {
                   Trans::Pop
               } else {
                   Trans::None
               }
            }
            _ => { Trans::None }
        }
    }
}

impl PauseState {
    pub fn initialise_ui(&mut self, world: &mut World) {
        world.exec(|mut creator: UiCreator<'_>| {
            self.ui_root = Some(creator.create("ui/pause/pauseOverlay.ron", ()));
        });
    }
}