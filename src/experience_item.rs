// This is a module for the experience item, when the player walks over it, the player's experience increases

use bevy::prelude::*;

#[derive(Component)]
pub struct ExperienceItem {
    pub experience: i32,
    pub size: Vec2,
}