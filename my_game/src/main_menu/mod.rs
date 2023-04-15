use bevy::prelude::*;

pub mod systems;
use systems::*;

pub struct GUIPlugin;

impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_gui);
        app.add_startup_system(main_gui);
    }
}

pub fn main_gui() {
    println!("Main menu");
}