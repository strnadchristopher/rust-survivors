// This is a bevy app and this is the player module, it handles moving the player with the keyboard and rendering the player

use bevy::prelude::*;
#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub position: (f32, f32),
    pub move_speed: f32,
    pub fire_rate: f32,
    pub size: Vec2,
    pub experience: i32,
    pub level: i32,
    pub experience_to_next_level: i32,
    pub recently_hit: bool,
}

#[derive(Component)]
pub struct PlayerHitTimer(pub Timer);

#[derive(Component)]
pub struct ProjectileTimer(pub Timer);

// Function for moving the player based on wasd keyboard input, include delta time for smooth movement
pub fn sprite_movement(time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player), Without<crate::enemy::Enemy>>,
) {
    for (mut transform, mut player) in player_query.iter_mut() {
        let mut x = transform.translation.x;
        let mut y = transform.translation.y;
        if keyboard_input.pressed(KeyCode::W) {
            y += player.move_speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::S) {
            y -= player.move_speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::A) {
            x -= player.move_speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::D) {
            x += player.move_speed * time.delta_seconds();
        }
        transform.translation.x = x;
        transform.translation.y = y;
        player.position = (x, y);
    }
}

// Function to check collision with enemies and the player, if the player is hit, we decrease his health
pub fn player_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &Player), Without<crate::enemy::Enemy>>,
    mut enemy_query: Query<(&Transform, &crate::enemy::Enemy), Without<Player>>,
    mut health_text_query: Query<&mut Text, With<crate::ui::HealthText>>
) {
    for (entity, player_transform, player) in player_query.iter_mut() {
        for (enemy_transform, _) in enemy_query.iter_mut() {
            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y;
            let enemy_x = enemy_transform.translation.x;
            let enemy_y = enemy_transform.translation.y;
            if player_x - player.size.x / 2.0 < enemy_x + 25.0
                && player_x + player.size.x / 2.0 > enemy_x - 25.0
                && player_y - player.size.y / 2.0 < enemy_y + 25.0
                && player_y + player.size.y / 2.0 > enemy_y - 25.0
            {
                // Check if player was recetently hit
                if player.recently_hit {
                    continue;
                }
                println!("Player hit!");
                // Update health text ui component
                for mut health_text in health_text_query.iter_mut() {
                    // Ui text component
                    health_text.sections[0].value = format!("Health: {}", player.health - 1);
                    
                }

                // Decrease player health
                commands.entity(entity).insert(Player {
                    health: player.health - 1,
                    recently_hit: true,
                    ..*player
                });
            }else{
            }
        }
    }
}

pub fn update_hit_timer(
    mut commands: Commands,
    time: Res<Time>, 
    mut player_query: Query<(Entity, &mut Player)>,
    mut player_hit_timer_query: Query<&mut PlayerHitTimer>,
)    {
    for (entity, mut player) in player_query.iter_mut() {
        for mut timer in player_hit_timer_query.iter_mut() {
            timer.0.tick(time.delta());
            if timer.0.finished() {
                commands.entity(entity).insert(Player {
                    recently_hit: false,
                    ..*player
                });
            }
        }
    }
}

// Function checks collision with experience items and the player, if the player collides with an experience item, we increase the player's experience, and despawn the experience item
pub fn experience_collision(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    mut experience_query: Query<(Entity, &Transform, &crate::experience_item::ExperienceItem)>,
    mut experience_text_query: Query<&mut Text, (With<crate::ui::ExperienceText>, Without<crate::ui::LevelText>)>,
    mut level_text_query: Query<&mut Text, (With<crate::ui::LevelText>, Without<crate::ui::ExperienceText>)>,
) {
    for mut player in player_query.iter_mut() {
        for (entity, experience_transform, experience_item) in experience_query.iter_mut() {
            let player_x = player.position.0;
            let player_y = player.position.1;
            let experience_x = experience_transform.translation.x;
            let experience_y = experience_transform.translation.y;
            if player_x - player.size.x / 2.0 < experience_x + 25.0
                && player_x + player.size.x / 2.0 > experience_x - 25.0
                && player_y - player.size.y / 2.0 < experience_y + 25.0
                && player_y + player.size.y / 2.0 > experience_y - 25.0
            {
                // Increase player experience
                player.experience += experience_item.experience;
                // If player experience is greater than or equal to the experience needed to level up, level up the player
                if player.experience >= player.experience_to_next_level {
                    player.level += 1;
                    player.experience = 0;
                    player.experience_to_next_level = player.experience_to_next_level * 2;
                }
                // Update experience text ui component
                for mut experience_text in experience_text_query.iter_mut() {
                    // Ui text component
                    experience_text.sections[0].value = format!("Experience: {}", player.experience);
                }
                // Update the player level text ui component
                for mut level_text in level_text_query.iter_mut() {
                    // Ui text component
                    level_text.sections[0].value = format!("Level: {}", player.level);
                }
                // Despawn experience item
                commands.entity(entity).despawn();
            }
        }
    }
    
}
