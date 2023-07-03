mod blocks;
mod control;
mod entities;
mod gamemap;
mod player;
mod render;
mod ui;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.insert_resource(gamemap::new_gamemap());
    app.add_plugin(render::RenderPlugin);
    app.add_plugin(entities::EntityUpdatePlugin);
    app.add_plugin(control::ControlPlugin);
    app.add_plugin(ui::UIPlugin);
    app.run();
}
