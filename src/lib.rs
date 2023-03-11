#![allow(clippy::needless_pass_by_value)]

mod camera_plugin;
mod drones;
mod spatial_hashmap;

pub use camera_plugin::CameraPlugin;
pub use drones::DronePlugin;
pub use spatial_hashmap::SpatialHashMap2D;

use bevy::prelude::{Component, Resource, Vec2};
use serde::Deserialize;

#[derive(Component, Default)]
struct Velocity(pub Vec2);

#[derive(Resource, Deserialize)]
pub struct GameConfig {
    pub drone_count: usize,
    pub spatial_map_cell_size: f32,
    pub cohesion_strength: f32,
}
