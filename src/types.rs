use gameconst::*;
use bevy::{input::{keyboard::KeyCode, Input}, prelude::Component};
use core::f32::consts::PI;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl Directions {
  pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
    let keys= match self {
      Directions::Up => [KeyCode::Up, KeyCode::D],
      Directions::Down => [KeyCode::Down, KeyCode::F],
      Directions::Left => [KeyCode::Left, KeyCode::J],
      Directions::Right => [KeyCode::Right, KeyCode::K],
    };
    keys.iter().any(|code| input.just_pressed(*code))
  }

  pub fn rotation(&self) -> f32 {
    match self {
      Directions::Up => PI * 0.5,
      Directions::Down => -PI * 0.5,
      Directions::Left => PI,
      Directions::Right => 0.,
    }
  }

  pub fn y(&self) -> f32 {
    match self {
      Directions::Up => 150.,
      Directions::Down => 50.,
      Directions::Left => -50.,
      Directions::Right => -150.,
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}

impl Speed {
  pub fn value(&self) -> f32 {
    BASE_SPEED * self.multiplier()
  }
  pub fn multiplier(&self) -> f32 {
    match self {
      Speed::Slow => 1.,
      Speed::Medium => 1.2,
      Speed::Fast => 1.5,
    }
  }
}

#[derive(Clone, Debug)]
pub struct ArrowTime {
    pub spawn_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}
impl ArrowTime {
  fn new(click_time: f64, speed: Speed, direction: Directions) -> Self {
    let speed_value = speed.value();
    Self {
      spawn_time: click_time - (DISTANCE / speed_value) as f64,
      speed,
      direction,
    }
  }
}

#[derive(Component)]
pub struct SongConfig {
    pub arrows: Vec<ArrowTime>,
}
pub fn load_config() -> SongConfig {
    SongConfig {
      arrows: vec![
        ArrowTime::new(1., Speed::Slow, Directions::Up),
        ArrowTime::new(2., Speed::Slow, Directions::Down),
        ArrowTime::new(3., Speed::Slow, Directions::Left),
        ArrowTime::new(4., Speed::Medium, Directions::Up),
        ArrowTime::new(5., Speed::Fast, Directions::Right),
      ]
    }
}