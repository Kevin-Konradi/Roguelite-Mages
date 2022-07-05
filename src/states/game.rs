use crate::{App, Commands, GameState, Plugin, SystemSet};

struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup))
            .add_loading_state(
                
            );
    }
}

fn setup(commands: Commands) {
    commands.
}