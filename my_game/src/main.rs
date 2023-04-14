use bevy::prelude::*;

mod player;
mod main_menu;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(main_menu::GUIPlugin)
        .run();
}
