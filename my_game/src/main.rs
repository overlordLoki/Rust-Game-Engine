use bevy::prelude::*;
use bevy_egui::*;
mod player;
mod main_menu;

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(main_menu::GUIPlugin)
        .add_system(ui_example_system)
        .run();
}
