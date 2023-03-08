use bevy::prelude::*;
use boids_tower_defense::{DronePlugin, GameConfig};

fn main() {
    let config: GameConfig = config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .expect("the `config` file should be in the root dir and it should be valid")
        .try_deserialize()
        .expect("the `config` should match the domain settings type");

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(config)
        .add_plugin(DronePlugin)
        .run();
}
