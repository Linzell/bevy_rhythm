use bevy::{input::system::exit_on_esc_system, prelude::*};

mod arrows;
use arrows::ArrowsPlugin;

mod types;
use types::SongConfig;

fn main() {
    App::new()
      .insert_resource(Msaa { samples: 4 })
      .insert_resource(WindowDescriptor {
          title: "Rhythm Game".to_string(),
          width: 800.,
          height: 600.,
          ..Default::default()
      })
      .add_startup_system(setup)
      .add_system(exit_on_esc_system)
      .add_plugins(DefaultPlugins)
      .add_plugin(ArrowsPlugin)
    .run();
}

fn setup(mut commands: Commands) {
  let config = types::load_config();
  commands
      .spawn_bundle(OrthographicCameraBundle::new_2d())
      .insert(SongConfig { arrows: config.arrows });
      //.commands()
      //.spawn_bundle(UiCameraBundle::default());
}