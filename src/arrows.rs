use bevy::prelude::*;
use gameconst::*;
use crate::types::*;

struct ArrowMaterialResource {
    red_texture: Handle<ColorMaterial>,
    blue_texture: Handle<ColorMaterial>,
    green_texture: Handle<ColorMaterial>,
    border_texture: Handle<ColorMaterial>,
}
impl FromWorld for ArrowMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let red_handle = asset_server.load("images/arrow_red.png");
        let blue_handle = asset_server.load("images/arrow_blue.png");
        let green_handle = asset_server.load("images/arrow_green.png");
        let border_handle = asset_server.load("images/arrow_border.png");
        ArrowMaterialResource {
            red_texture: materials.add(red_handle.into()),
            blue_texture: materials.add(blue_handle.into()),
            green_texture: materials.add(green_handle.into()),
            border_texture: materials.add(border_handle.into()),
        }
    }
}

struct Arrow {
    speed: Speed,
    direction: Directions,
}

struct SpawnTimer(Timer);

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<ArrowMaterialResource>()
            .add_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
            .add_system(spawn_arrows.system())
            .add_system(move_arrows.system());
    }
}

fn spawn_arrows(
    commands: &mut Commands,
    time: Res<Time>,
    mut song_config: ResMut<SongConfig>,
    materials: Res<ArrowMaterialResource>,
) {
  let secs = time.seconds_since_startup() - 3.;
  let secs_last = secs - time.delta_seconds_f64();

  let mut remove_counter = 0;
  for arrow in &song_config.arrows {
    if secs_last < arrow.spawn_time && arrow.spawn_time < secs {
      remove_counter += 1;

      let material = match arrow.speed {
        Speed::Slow => &materials.green_texture,
        Speed::Medium => &materials.blue_texture,
        Speed::Fast => &materials.red_texture,
      };

      let mut transform = 
        Transform::from_translation(Vec3::new(SPAWN_POSITION, arrow.direction.y(), 1.));
      transform.rotate(Quat::from_rotation_z(arrow.direction.rotation()));
      commands
        .spawn_bundle(SpriteBundle {
          texture: material,
          sprite: Sprite::new(Vec2::new(140., 140.)),
          transform,
          ..Default::default()
        })
        .with(Arrow {
          speed: arrow.speed,
          direction: arrow.direction,
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