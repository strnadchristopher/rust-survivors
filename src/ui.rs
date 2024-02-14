// UI module for the game, the ui will show the player's health, experience, and level

use bevy::prelude::*;

#[derive(Component)]
pub struct UI;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct ExperienceText;

#[derive(Component)]
pub struct LevelText;

pub fn spawn_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn the health text
    commands.spawn((
        TextBundle::from_section(
            "Health: 100",
            TextStyle {
                font_size: 40.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        HealthText,
    ));

    // Spawn the experience text
    commands.spawn((
        TextBundle::from_section(
            "Experience: 0",
            TextStyle {
                font_size: 40.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        }),
        ExperienceText,
    ));

    // Spawn the Level text
    commands.spawn((
        TextBundle::from_section(
            "Level: 1",
            TextStyle {
                font_size: 40.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            right: Val::Px(100.0),
            ..default()
        }),
        LevelText,
    ));
}
