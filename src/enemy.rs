use bevy::prelude::*;
use crate::player;
use bevy::render::render_resource::Texture;


// Enemy component

#[derive(Component)]
pub struct Enemy{
    pub move_speed: f32,
    pub size: Vec2,
    pub health: i32,
}

#[derive(Component)]
pub struct SpawnEnemyTimer(pub Timer);

// Spawn enemies on a timer
pub fn spawn_enemy(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer_query: Query<&mut SpawnEnemyTimer>,
    // We'll need to spawn the enemy outside the viewport, so we'll need to query for the camera
    camera_query: Query<&Transform, With<crate::camera::Camera>>,
) {
    for mut timer in timer_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            // Enemy x should be a random location outside the viewport
            // That means setting the x to either 
            // So should enemy y
            let camera_x = camera_query.single().translation.x;
            let camera_y = camera_query.single().translation.y;
            let viewport_width = 800.0;
            let viewport_height = 600.0;
            let enemy_x = match rand::random(){
                true => {
                    camera_x + viewport_width + 50.0
                },
                false =>{
                    camera_x - viewport_width - 50.0
                }
            };
            let enemy_y = match rand::random(){
                true => {
                    camera_y + viewport_height + 50.0
                },
                false =>{
                    camera_y - viewport_height - 50.0
                }
            };
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("branding/howl.png"),
                    transform: Transform::from_xyz(enemy_x, enemy_y, 0.0),
                    ..Default::default()
                },
                Enemy {
                    move_speed: 75.0,
                    size: Vec2::new(50.0, 50.0),
                    health: 3,
                },
            ));
        }
    }
}
// Function to make all enemies move towards the player, based on move speed, include delta time for smooth movement
pub fn move_enemy(
    time: Res<Time>,
    player_query: Query<(&player::Player, &Transform), (With<player::Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Transform, &Enemy), Without<player::Player>>,
) {
    for (_, player_transform) in player_query.iter() {
        for (mut transform, enemy) in enemy_query.iter_mut() {
            let mut x = transform.translation.x;
            let mut y = transform.translation.y;
            let x_diff = player_transform.translation.x - x;
            let y_diff = player_transform.translation.y - y;
            let distance = (x_diff.powi(2) + y_diff.powi(2)).sqrt();
            let x_speed = x_diff / distance * enemy.move_speed;
            let y_speed = y_diff / distance * enemy.move_speed;
            x += x_speed * time.delta_seconds();
            y += y_speed * time.delta_seconds();
            transform.translation.x = x;
            transform.translation.y = y;
        }
    }
}

// Now we check for collision with projectiles and the enemy, if the enemy is hit, we destroy the enemy and the projectile
pub fn enemy_collision(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform, &Sprite, &Enemy), With<Enemy>>,
    mut projectile_query: Query<(Entity, &Transform, &Sprite), With<crate::projectile::Projectile>>,
) {
    for (enemy_entity, enemy_transform, _, enemy) in enemy_query.iter_mut() {
        for (projectile_entity, projectile_transform, _) in projectile_query.iter_mut() {
            let enemy_x = enemy_transform.translation.x;
            let enemy_y = enemy_transform.translation.y;
            let projectile_x = projectile_transform.translation.x;
            let projectile_y = projectile_transform.translation.y;
            if enemy_x - enemy.size.x / 2.0 < projectile_x + 5.0
                && enemy_x + enemy.size.x / 2.0 > projectile_x - 5.0
                && enemy_y - enemy.size.y / 2.0 < projectile_y + 5.0
                && enemy_y + enemy.size.y / 2.0 > projectile_y - 5.0
            {
                commands.entity(projectile_entity).despawn();
                // Check if the enemy is dead
                if enemy.health <= 1 {
                    commands.entity(enemy_entity).despawn();
                    // Then spawn an experience_item
                    commands.spawn(
                        (
                            SpriteBundle {
                                texture: asset_server.load("branding/gem.png"),
                                transform: Transform::from_xyz(enemy_x, enemy_y, 0.0),
                                ..Default::default()
                            },
                            crate::experience_item::ExperienceItem {
                                size: Vec2::new(10.0, 10.0),
                                experience: 10,
                            },
                        )
                    );
                } else {
                    commands.entity(enemy_entity).insert(Enemy {
                        health: enemy.health - 1,
                        ..*enemy
                    });
                }
            }
        }
    }
}