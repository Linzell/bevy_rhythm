use bevy::prelude::*;

/// Keeps the textures and materials for Arrows
struct ArrowMaterialResource {
    red_texture: Handle<ColorMaterial>,
    blue_texture: Handle<ColorMaterial>,
    green_texture: Handle<ColorMaterial>,
    border_texture: Handle<ColorMaterial>,
}
impl FromResources for ArrowMaterialResource {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get::<AssetServer>().unwrap();

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

struct Arrow;

struct SpawnTimer(Timer);

fn spawn_arrows(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    materials: Res<ArrowMaterialResource>,
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..1280.0);
        let y = rng.gen_range(0.0..720.0);
        let color = rng.gen_range(0..3);
        let texture = match color {
            0 => materials.red_texture.clone(),
            1 => materials.blue_texture.clone(),
            2 => materials.green_texture.clone(),
            _ => panic!("Invalid color"),
        };
        commands
            .spawn_bundle(SpriteBundle {
                material: texture,
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            })
            .insert(Arrow);
    }
}