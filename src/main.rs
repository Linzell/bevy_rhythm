use std::sync::Arc;

use bevy::{input::system::exit_on_esc_system, prelude::*};

mod arrows;
use arrows::ArrowsPlugin;

mod types;

fn main() {
  Arc::new(App::new()
      .add_startup_system(setup)
      .add_plugins(DefaultPlugins)
      .add_system(exit_on_esc_system.system()))
      .add_plugins(ArrowsPlugin)
    .run();
}

fn setup(mut commands: Commands) {
  let config = types::load_config();
  commands
      .spawn_bundle(OrthographicCameraBundle::new_2d())
      .insert_resource(config);
}