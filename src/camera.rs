// This is the camera module, the camera follows the player
use bevy::{ecs::query::WorldQuery, prelude::*};
use crate::player;
use crate::enemy;

#[derive(Component, WorldQuery)]
pub struct Camera;

pub fn create_camera(commands: &mut Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        Camera,
    ));
}

// Function to make the camera center on the player, there are also enemy components in the game, so we need to exclude them
pub fn follow_player(
    player_query: Query<(&player::Player, &Transform), (With<player::Player>, Without<enemy::Enemy>, Without<Camera>)>,
    mut camera_query: Query<(&mut Transform, &Camera)>,
) {
    for (_, player_transform) in player_query.iter() {
        for (mut transform, _) in camera_query.iter_mut() {
            transform.translation.x = player_transform.translation.x;
            transform.translation.y = player_transform.translation.y;
        }
    }
}
