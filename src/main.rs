mod blocks;
mod control;
mod entities;
pub mod gamemap;
mod player;
mod render;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(render::RenderPlugin);
    app.add_plugin(entities::EntityUpdatePlugin);
    app.add_plugin(control::ControlPlugin);
    app.insert_resource(gamemap::load_gamemap(""));
    app.run();
}
