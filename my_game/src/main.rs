use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// constants
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
#[derive(Component,Clone)]
pub struct Player {
    pub name: String,
    //x y z position
    x : f32,
    y : f32,
    z : f32,
}

fn spawn_player(
    mut commands: Commands, asset_server: Res<AssetServer>,window_query: Query<&Window, With<PrimaryWindow>>,)
{
    let window = window_query.get_single().unwrap();
    let player = Player{
        name: "Player 1".to_string(),
        x: window.width() as f32 / 2.0,
        y: window.height() as f32 / 2.0,
        z: 0.0,
    };
    println!("Player spawned {}" , player.name);
    commands.spawn((
        SpriteBundle {
            transform: Transform:: from_xyz(player.x, player.y, player.z),
            texture: asset_server.load("set3/PNG/Retina/ball_blue_small.png"),
            ..default()
        },
        player,
    ));
    
}

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

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,mut player_query: Query<&mut Transform, With<Player>>, time: Res<Time>)
     {

    let mut transform: Mut <Transform> = player_query.single_mut();
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }
    transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
}

pub fn confine_player_to_screen(
    mut player_query: Query<&mut Transform, With<Player>>, window_query: Query<&Window, With<PrimaryWindow>>)
    {
    let window = window_query.get_single().unwrap();
    let mut transform: Mut <Transform> = player_query.single_mut();
    let mut player_x = transform.translation.x;
    let mut player_y = transform.translation.y;
    //half the player size
    let half_player_size = PLAYER_SIZE / 2.0;
    //confine player to screen
    if player_x - half_player_size < 0.0 {
        player_x = half_player_size;
    }
    if player_x + half_player_size > window.width() as f32 {
        player_x = window.width() as f32 - half_player_size;
    }
    if player_y - half_player_size < 0.0 {
        player_y = half_player_size;
    }
    if player_y + half_player_size > window.height() as f32 {
        player_y = window.height() as f32 - half_player_size;
    }
    transform.translation.x = player_x;
    transform.translation.y = player_y;

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
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .add_system(confine_player_to_screen)
        .run();
}

