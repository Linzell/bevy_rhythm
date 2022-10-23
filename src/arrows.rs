use bevy::{prelude::*};
use gameconst::*;
use crate::types::*;

struct ArrowMaterialResource {
    red_texture: Handle<Image>,
    blue_texture: Handle<Image>,
    green_texture: Handle<Image>,
    border_texture: Handle<Image>,
}

impl FromWorld for ArrowMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let red_texture = asset_server.load("images/arrow_red.png");
        let blue_texture = asset_server.load("images/arrow_blue.png");
        let green_texture = asset_server.load("images/arrow_green.png");
        let border_texture = asset_server.load("images/arrow_border.png");
        Self {
            red_texture,
            blue_texture,
            green_texture,
            border_texture,
        }
    }
}

#[derive(Component)]
struct Arrow {
    speed: Speed,
    direction: Directions,
}

struct SpawnTimer(Timer);

fn spawn_arrows(
    mut commands: Commands,
    mut song_config: ResMut<SongConfig>,
    materials: Res<ArrowMaterialResource>,
    time: Res<Time>,
) {
  let secs = time.seconds_since_startup() - 3.;
  let secs_last = secs - time.delta_seconds_f64();

  let mut remove_counter = 0;
  for arrow in &song_config.arrows {
    if secs_last < arrow.spawn_time && arrow.spawn_time < secs {
      remove_counter += 1;

      let material = match arrow.speed {
        Speed::Slow => materials.green_texture.clone(),
        Speed::Medium => materials.blue_texture.clone(),
        Speed::Fast => materials.red_texture.clone(),
      };

      let mut transform = 
        Transform::from_translation(Vec3::new(SPAWN_POSITION, arrow.direction.y(), 1.));
      transform.rotate(Quat::from_rotation_z(arrow.direction.rotation()));
      commands
        .spawn_bundle(SpriteBundle {
          texture: material,
          sprite: Sprite {
            custom_size: Option::Some(Vec2::new(140., 140.)),
            ..Default::default()
          },
          transform,
          ..Default::default()
        })
        .insert(Arrow {
          speed: arrow.speed.clone(),
          direction: arrow.direction.clone(),
        });
    } else {
      break;
    }
  }

  for _ in 0..remove_counter {
    song_config.arrows.remove(0);
  }
}

fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
  for (mut transform, arrow) in query.iter_mut() {
    transform.translation.x += time.delta_seconds() * arrow.speed.value();
  }
}

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
          // Initialize Resources
          .init_resource::<ArrowMaterialResource>()
          .insert_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
          // Add systems
          .add_system(spawn_arrows)
          .add_system(move_arrows);
    }
}