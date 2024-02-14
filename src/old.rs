use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy::log::LogPlugin;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update,(sprite_movement, move_enemy));
    }
}

// This is a simple rust bevy game that moves a sprite around the world and has the camera follow it
// The game also renders enemies and has those enemies move towards the player constantly
// When the enemies collide with the player, the player is destroyed and the game ends

fn main() {
    App::new().add_plugins((DefaultPlugins.set(
        LogPlugin {filter: "info,wgpu_core=off,wgpu_hal=off,mygame=debug".into(),
    level: bevy::log::Level::INFO,}
    ), HelloPlugin)).run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Component, PartialEq)]

enum CharacterType{
    Player,
    Enemy,
}

#[derive(Component)]
struct Player{
    health: i32,
    position: (f32, f32)
}

impl Player{
    fn spawn(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>){
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("branding/icon.png"),
                transform: Transform{
                    translation: Vec3::new(self.position.0, self.position.1, 0.0),
                    scale: Vec3::new(0.25, 0.25, 0.25),
                    ..Default::default()
                },
                ..default()
            },
            CharacterType::Player,
        ));
    }
}

#[derive(Component)]
struct Enemy{
    health: i32,
    position: (f32, f32)
}

impl Enemy{
    fn spawn(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>){
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("branding/icon.png"),
                transform: Transform{
                    translation: Vec3::new(self.position.0, self.position.1, 0.0),
                    scale: Vec3::new(0.25, 0.25, 0.25),
                    ..Default::default()
                },
                ..default()
            },
            CharacterType::Enemy,
        ));
    }

}



fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a camera to follow the player
    commands.spawn(Camera2dBundle::default());
    // Create player
    Player::spawn(&mut Player{health: 100, position: (0.0, 0.0)}, &mut commands, &asset_server);
    // Create enemy
    Enemy::spawn(&mut Enemy{health: 100, position: (100.0, 100.0)}, &mut commands, &asset_server);
}

// Function to move player around and have camera follow it, use WASD controls
fn sprite_movement(
    mut query: Query<(&mut Transform, &Player, &mut Direction)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>, Without<Enemy>)>,

) {
    for (mut transform, _player, mut direction) in query.iter_mut() {
        let mut x = 0.0;
        let mut y = 0.0;
        match *direction {
            Direction::Up => y = 1.0,
            Direction::Down => y = -1.0,
            Direction::Left => x = -1.0,
            Direction::Right => x = 1.0,
            Direction::None => {}
        }
        transform.translation.x += x;
        transform.translation.y += y;
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation.x = transform.translation.x;
            camera_transform.translation.y = transform.translation.y;
        }
    }
}

// Functions to move all enemies toward player
fn move_enemy(
    mut query: Query<(&mut Transform, &Enemy, &Player)>,
    mut player_query: Query<&Transform,( With<Player>, Without<Enemy>)>,
) {
    for (mut transform, _enemy, _player) in query.iter_mut() {
        for player_transform in player_query.iter() {
            let x = player_transform.translation.x - transform.translation.x;
            let y = player_transform.translation.y - transform.translation.y;
            let magnitude = (x * x + y * y).sqrt();
            let x = x / magnitude;
            let y = y / magnitude;
            transform.translation.x += x;
            transform.translation.y += y;
        }
    }
}

// Add function to have camera follow player
// fn camera_follow_player(
//     mut query: Query<(&Transform, &Player)>,
//     mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>, Without<Enemy>)>,
// ) {
//     for (transform, _player) in query.iter_mut() {
//         for mut camera_transform in camera_query.iter_mut() {
//             camera_transform.translation.x = transform.translation.x;
//             camera_transform.translation.y = transform.translation.y;
//         }
//     }
// }