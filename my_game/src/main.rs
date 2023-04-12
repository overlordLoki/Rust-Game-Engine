use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod player;

pub fn spawn_camera(
    mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>)
    {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
            ..default()}
    );
    println!("Camera spawned");
}


pub fn confine_object_to_box(
    object_x: f32, object_y: f32, object_width: f32, object_height: f32, box_x: f32, box_y: f32, box_width: f32, box_height: f32)
    -> (f32, f32)
    {
    let mut new_x = object_x;
    let mut new_y = object_y;
    let half_object_width = object_width / 2.0;
    let half_object_height = object_height / 2.0;
    let half_box_width = box_width / 2.0;
    let half_box_height = box_height / 2.0;
    //left
    if object_x - half_object_width < box_x - half_box_width {
        new_x = box_x - half_box_width + half_object_width;
    }
    //right
    if object_x + half_object_width > box_x + half_box_width {
        new_x = box_x + half_box_width - half_object_width;
    }
    //top
    if object_y + half_object_height > box_y + half_box_height {
        new_y = box_y + half_box_height - half_object_height;
    }
    //bottom
    if object_y - half_object_height < box_y - half_box_height {
        new_y = box_y - half_box_height + half_object_height;
    }
    return (new_x, new_y)
}



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

