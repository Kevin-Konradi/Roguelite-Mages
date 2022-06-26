use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans};
use amethyst::core::{SystemDesc, Transform};
use amethyst::core::alga::general::SupersetOf;
use amethyst::ecs::{Builder, Entity, Read, ReaderId, System, SystemData, World, WorldExt, Write};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::renderer::Camera;
use amethyst::shrev::EventChannel;
use amethyst::ui::{UiCreator, UiEvent, UiEventType, UiFinder};
use amethyst::window::ScreenDimensions;
use amethyst::winit::VirtualKeyCode;
use crate::states::menu_to_lobby_loading::MenuToLobbyLoadingState;

const BUTTON_START: &str = "button_start";
const BUTTON_EXIT: &str = "button_exit";

#[derive(Default)]
pub struct MainMenu {
    exit_button: Option<Entity>,
    start_button: Option<Entity>,
    ui_root: Option<Entity>,
}

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
        self.initialise_buttons(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // deletes the root ui element for this state, which essentially clears the screen.
        if let Some(entity) = self.ui_root {
            world.delete_entity(entity);
        }
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                match ui_event.event_type {
                    UiEventType::Click if Some(ui_event.target) == self.exit_button => {
                        Trans::Quit
                    }
                    UiEventType::Click if Some(ui_event.target) == self.start_button => {
                        Trans::Switch(Box::new(MenuToLobbyLoadingState::default()))
                    }
                    _ => { Trans::None }
                }
            }
            _ => { Trans::None }
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &mut data.world;

        if self.exit_button.is_none()
            || self.start_button.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.exit_button = ui_finder.find(BUTTON_EXIT);
                self.start_button = ui_finder.find(BUTTON_START);
            });
        }

        Trans::None
    }
}

fn initialise_camera(world: &mut World) {
    let (height, width) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.height(), dim.width())
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}

impl MainMenu {
    pub fn initialise_buttons(&mut self, world: &mut World) {
        world.exec(|mut creator: UiCreator<'_>| {
            self.ui_root = Some(creator.create("ui/mainMenu/mainMenuSplash.ron", ()));
        });
    }
}

pub struct MainMenuButtonSystem {
    reader_id: ReaderId<UiEvent>,
}

impl<'s> System<'s> for MainMenuButtonSystem {
    type SystemData = Read<'s, EventChannel<UiEvent>>;

    fn run(&mut self, events: Self::SystemData) {
        for event in events.read(&mut self.reader_id) {

        }
    }
}

impl MainMenuButtonSystem {
    pub fn new(reader_id: ReaderId<UiEvent>) -> Self {
        Self {
            reader_id,
        }
    }
}

#[derive(Default)]
pub struct MainMenuButtonSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, MainMenuButtonSystem> for MainMenuButtonSystemDesc {
    fn build(self, world: &mut World) -> MainMenuButtonSystem {
        let mut event_channel = <Write<EventChannel<UiEvent>>>::fetch(world);
        let reader_id = event_channel.register_reader();

        MainMenuButtonSystem::new(reader_id)
    }
}