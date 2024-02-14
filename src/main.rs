use bevy::log::LogPlugin;
use bevy::prelude::*;

// import lib.rs
use rust_survivors::camera;
use rust_survivors::enemy;
use rust_survivors::player;
use rust_survivors::projectile;
use rust_survivors::ui;
// import player module
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(LogPlugin {
                filter: "info,wgpu_core=off,wgpu_hal=off,mygame=debug".into(),
                level: bevy::log::Level::INFO,
            }),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player::sprite_movement,
                enemy::move_enemy,
                camera::follow_player,
                projectile::fire_projectile,
                projectile::update_projectiles,
                enemy::enemy_collision,
                enemy::spawn_enemy,
                player::player_collision,
                player::update_hit_timer,
                player::experience_collision
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    camera::create_camera(&mut commands);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            // Scale the player to half size
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(0.5, 0.5, 0.5),
            },
            ..default()
        },
        player::Player {
            health: 100,
            position: (100., 0.),
            move_speed: 100.,
            fire_rate: 4.,
            size: Vec2::new(50., 50.),
            experience: 0,
            level: 1,
            experience_to_next_level: 10,
            recently_hit: false,
        },
    ));


    // Setup projectile firing timer, every 4 seconds
    commands.spawn(projectile::ProjectileTimer(Timer::from_seconds(
        4.0,
        TimerMode::Repeating,
    )));

    // Setup enemy spawn timer, every 2 seconds
    commands.spawn(enemy::SpawnEnemyTimer(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    )));

    // Setup player hit timer
    commands.spawn(player::PlayerHitTimer(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    )));

    // Setup the ui, this will spawn
    ui::spawn_ui(&mut commands, asset_server);

}
