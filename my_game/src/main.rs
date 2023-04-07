use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
struct Player {
    pub name: String,
    //x y z position
    x : f32,
    y : f32,
    z : f32,
}

fn spawn_player(
    mut commands: Commands, asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ){
    let window = window_query.get_single().unwrap();
    let player = Player{
        name: "Player_1".to_string(),
        x: window.width() as f32 / 2.0,
        y: window.height() as f32 / 2.0,
        z: 0.0,
    };
    commands.spawn((
        SpriteBundle {
            transform: Transform:: from_xyz(player.x, player.y, player.z),
            texture: asset_server.load("set3/PNG/Retina/ball_blue_large.png"),
            ..default()
        },
        player,
    ));
    println!("Player spawned ");
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>){
    let window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
            ..default()}
    );
    println!("Camera spawned");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .run();
}

