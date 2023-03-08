#![allow(clippy::needless_pass_by_value)]

mod drones;

pub use drones::DronePlugin;

use bevy::prelude::{Component, Resource, Vec2};
use serde::Deserialize;

#[derive(Component, Default)]
struct Velocity(pub Vec2);

#[derive(Resource, Deserialize)]
pub struct GameConfig {
    pub drone_count: usize,
}
