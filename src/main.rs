mod blocks;
mod control;
mod entities;
mod gamemap;
mod init_game;
mod interaction;
mod player;
mod ui;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

/**
The enum that represents the state of the game. This is a global resource.
 */
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    InGame,
    #[default]
    MainMenu,
    Pause,
    Loading,
}

/**
The plugin group that is used in game,
including render, control, collision, and entity update.
 */
struct InGamePluginGroup;
impl PluginGroup for InGamePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        let builder = PluginGroupBuilder::start::<Self>();
        let builder = builder.add(init_game::InitGamePlugin);
        let builder = builder.add(control::ControlPlugin);
        let builder = builder.add(entities::EntityUpdatePlugin);
        let builder = builder.add(interaction::InteractionPlugin);
        return builder;
    }
}

fn main() {
    let mut app = App::new();
    app.add_state::<GameState>();
    app.init_resource::<gamemap::GameMap>();
    app.init_resource::<gamemap::WorldName>();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "RustMC!".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.add_plugins(InGamePluginGroup);
    app.add_plugin(ui::UIPlugin);
    app.run();
}
