
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::{Player};


// constants
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

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

pub fn spawn_player(
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