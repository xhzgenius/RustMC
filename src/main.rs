mod blocks;
mod control;
mod entities;
mod gamemap;
mod player;
mod render;
mod ui;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

/**
The plugin group that is used in game, 
including render, control, collision, entity update,
and in-game UI.
 */
struct GamePluginGroup;
impl PluginGroup for GamePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(render::RenderPlugin)
            .add(control::ControlPlugin)
            .add(entities::EntityUpdatePlugin)
            .add(ui::GameUIPlugin)
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(gamemap::load_gamemap("./saves/test_gamemap.json"))
        .add_plugins(GamePluginGroup);

    app.run();
}
