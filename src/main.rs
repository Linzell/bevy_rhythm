use bevy::{input::system::exit_on_esc_system, prelude::*};
/* use bevy::prelude::Query; */

/* struct Position {
    x: f32,
    y: f32,
}

struct Name(String);

struct Person;

struct Scoreboard {
    score: usize,
}

#[derive(Default)]
struct OtherScore(f32); */

fn main() {
  App::new()
    .add_startup_system(setup)
    .add_plugins(DefaultPlugins)
    .add_system(exit_on_esc_system.system())
    /* .add_resource(Scoreboard { score: 7 })
    .init_resource::<OtherScore>()

    .add_startup_system(setup.system())
    .add_system(set_names.system())
    .add_system(change_position.system()) */
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

/* fn set_names(mut query: Query<(&Position, &mut Name), With<Person>>) {
    for (pos, mut name) in query.iter_mut() {
        name.0 = format!("position: ({}, {})", pos.x, pos.y);
    }
}

fn setup(commands: &mut Commands) {
    commands
        .spawn((Position { x: 1., y: 2. }, Name("Entity 1".to_string())))
        .spawn((Position { x: 3., y: 9. }, Name("Entity 2".to_string())));
}

fn change_position(mut query: Query<&mut Position>, time: Res<Time>) {
    for mut pos in query.iter_mut() {
        pos.x += time.seconds_since_startup() as f32;
    }
} */