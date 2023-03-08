use crate::{GameConfig, Velocity};
use bevy::prelude::*;

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
    transform_bundle: TransformBundle,
}

// ------------------------ Systems

fn spawn(mut commands: Commands, settings: Res<GameConfig>) {
    info!("Spawning {} drones.", settings.drone_count);
    let drones_iter = (0..settings.drone_count).map(|_| DroneBundle::default());
    commands.spawn_batch(drones_iter);
}
