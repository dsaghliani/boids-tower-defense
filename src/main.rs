use bevy::prelude::*;
use boids_tower_defense::{CameraPlugin, DronePlugin, GameConfig};

fn main() {
    let config = get_configuration()
        .expect("the configuration should be present and valid");

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(config)
        .add_plugin(CameraPlugin)
        .add_plugin(DronePlugin)
        .run();
}

fn get_configuration() -> Result<GameConfig, config::ConfigError> {
    config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?
        .try_deserialize()
}
