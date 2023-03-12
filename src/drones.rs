use crate::{GameConfig, SpatialHashMap2D, Velocity};
use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;
use std::f32::consts;

// ------------------------ Plugin

pub struct DronePlugin;

impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_spatial_map)
            .add_startup_system(spawn_drones)
            .add_systems((update_spatial_map, update_drones).chain());
    }
}

// ------------------------ Data

#[derive(Component, Default)]
struct Drone;

#[derive(Bundle, Default)]
struct DroneBundle {
    drone: Drone,
    velocity: Velocity,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

/// A tuple of (`Entity`, position, velocity).
type DroneData = (Entity, Vec2, Vec2);

// ------------------------ Systems

fn spawn_spatial_map(mut commands: Commands, settings: Res<GameConfig>) {
    commands.spawn(SpatialHashMap2D::<DroneData>::new(
        settings.spatial_map_cell_size,
    ));
}

fn spawn_drones(
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
                sprite: Sprite {
                    color: Color::ORANGE_RED,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

fn update_spatial_map(
    drones: Query<(Entity, &Transform, &Velocity), With<Drone>>,
    mut spatial_map: Query<&mut SpatialHashMap2D<DroneData>>,
) {
    let mut spatial_map = spatial_map.single_mut();
    spatial_map.clear();

    for (id, transform, velocity) in &drones {
        let position = transform.translation.truncate();
        spatial_map.add(position, (id, position, velocity.0));
    }
}

fn update_drones(
    mut drones: Query<(&mut Velocity, &mut Transform, Entity), With<Drone>>,
    mut spatial_map: Query<&mut SpatialHashMap2D<DroneData>>,
    settings: Res<GameConfig>,
    time: Res<Time>,
) {
    let mut spatial_map = spatial_map.single_mut();

    for (mut velocity, mut transform, id) in &mut drones {
        let position = transform.translation.truncate();
        velocity.0 +=
            cohesion(id, position, &mut spatial_map) * settings.cohesion_strength;
        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
    }
}

// ------------------------ RULES

fn cohesion(
    id: Entity,
    position: Vec2,
    context: &mut SpatialHashMap2D<DroneData>,
) -> Vec2 {
    debug!("Calculating cohesion for Drone: {id:?}.");

    let mut neighbor_count = 0;
    let mut position_sum = Vec2::ZERO;

    for (other_id, other_position, _) in context
        .neighbors(position)
        .into_iter()
        .filter(|(other_id, _, _)| id != *other_id)
    {
        trace!("\tCalculating cohesion against {other_id:?}.");
        neighbor_count += 1;
        position_sum += other_position;
    }

    if neighbor_count > 0 {
        #[allow(clippy::cast_precision_loss)]
        let center_of_mass = position_sum / neighbor_count as f32;
        center_of_mass - position
    } else {
        Vec2::ZERO
    }
}
