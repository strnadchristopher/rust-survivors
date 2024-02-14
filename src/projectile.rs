use crate::enemy;
use crate::player;
use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub x_speed: f32,
    pub y_speed: f32,
    pub size: Vec2,
}

#[derive(Component)]
pub struct ProjectileTimer(pub Timer);

// Fire a projectile from the player's position, assuming the timer is finished
pub fn fire_projectile(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<
        (&player::Player, &Transform),
        (With<player::Player>, Without<enemy::Enemy>),
    >,
    // Query projectile timer
    mut projectile_timer_query: Query<&mut ProjectileTimer>,
    enemy_query: Query<(&Transform, &enemy::Enemy), Without<player::Player>>,
) {
    // If the projectile timer is finished, fire a projectile
    for mut timer in projectile_timer_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            for (_, player_transform) in player_query.iter() {
                // Find the closest enemy to the player, than fire a projectile at it
                let mut closest_enemy = None;
                let mut closest_distance = f32::MAX;
                for (enemy_transform, _) in enemy_query.iter() {
                    let x_diff = enemy_transform.translation.x - player_transform.translation.x;
                    let y_diff = enemy_transform.translation.y - player_transform.translation.y;
                    let distance = (x_diff.powi(2) + y_diff.powi(2)).sqrt();
                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_enemy = Some((x_diff, y_diff));
                    }
                }
                if let Some((x_diff, y_diff)) = closest_enemy {
                    let distance = (x_diff.powi(2) + y_diff.powi(2)).sqrt();
                    let x_speed = x_diff / distance * 200.0;
                    let y_speed = y_diff / distance * 200.0;
                    commands.spawn((
                        SpriteBundle {
                            texture: asset_server.load("branding/projectile.png"),
                            transform: Transform {
                                translation: player_transform.translation,
                                rotation: Quat::IDENTITY,
                                scale: Vec3::new(0.1, 0.1, 0.1),
                            },
                            ..Default::default()
                        },
                        Projectile {
                            x_speed,
                            y_speed,
                            size: Vec2::new(5.0, 5.0),
                        },
                    ));
                }

                
            }
        }
    }
}


pub fn update_projectiles(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Projectile)>,
) {
    for (_, mut transform, projectile) in query.iter_mut() {
        transform.translation.x += projectile.x_speed * time.delta_seconds();
        transform.translation.y += projectile.y_speed * time.delta_seconds();
    }
}