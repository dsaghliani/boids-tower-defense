use crate::{GameConfig, Velocity};
use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;
use std::f32::consts;

// ------------------------ Plugin

pub struct DronePlugin;

impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn);
    }
}

// ------------------------ Components

#[derive(Component, Default)]
struct Drone;

#[derive(Bundle, Default)]
struct DroneBundle {
    drone: Drone,
    velocity: Velocity,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

// ------------------------ Systems

fn spawn(
    mut commands: Commands,
    settings: Res<GameConfig>,
    asset_server: Res<AssetServer>,
    primary_win: Query<&Window, With<PrimaryWindow>>,
) {
    debug!("Spawning {} drones.", settings.drone_count);

    let (win_width, win_height) = {
        let primary_win = primary_win.single();
        (primary_win.width(), primary_win.height())
    };
    let mut rng = rand::thread_rng();

    for _ in 0..settings.drone_count {
        let texture = asset_server.load("boid_16.png");
        let translation = Vec3::new(
            rng.gen_range(-win_width / 2.0..win_width / 2.0),
            rng.gen_range(-win_height / 2.0..win_height / 2.0),
            0.0,
        );
        let rotation =
            Quat::from_rotation_z(rng.gen_range(0.0..(consts::PI * 2.0)));

        trace!(
            "Spawning a `DroneBundle` at translation {} and rotation {} with the \
                texture {:?}.",
            translation,
            rotation,
            texture
        );

        commands.spawn(DroneBundle {
            sprite_bundle: SpriteBundle {
                texture,
                transform: Transform {
                    translation,
                    rotation,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
