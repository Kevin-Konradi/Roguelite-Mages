use amethyst::ecs::{ReaderId, System};
use amethyst::ui::UiEvent;

pub struct MainMenuUiEventHandlerSystem {
    reader_id: ReaderId<UiEvent>,
}

impl MainMenuUiEventHandlerSystem {
    pub fn new(reader_id: ReaderId<UiEvent>) -> Self {
        Self {reader_id}
    }
}

impl<'s> System<'s> for MainMenuUiEventHandlerSystem {
    type SystemData = (

    );

    fn run(&mut self, data: Self::SystemData) {
        todo!()
    }
}