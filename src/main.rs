mod player;
pub mod gamemap;
mod entities;
mod blocks;
mod render;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(render::RenderPlugin);
    app.insert_resource(gamemap::load_gamemap(""));
    app.run();
}
