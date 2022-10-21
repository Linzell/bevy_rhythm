use gameconst::*;
use bevy::input::{keyboard::KeyCode, Input};
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
      Directions::Up => [KeyCode::Up, KeyCode::D] || [KeyCode::Up],
      Directions::Down => [KeyCode::Down, KeyCode::F] || [KeyCode::Down],
      Directions::Left => [KeyCode::Left, KeyCode::J] || [KeyCode::Left],
      Directions::Right => [KeyCode::Right, KeyCode::K] || [KeyCode::Right],
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

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct ArrowTime {
    pub direction: Directions,
    pub speed: Speed,
    pub spawn_time: f64,
}
impl ArrowTime {
  pub fn new(direction: Directions, speed: Speed, click_time: f64) -> Self {
    Self {
      direction,
      speed,
      spawn_time: click_time - (DISTANCE / speed.value()) as f64,
    }
  }
}

#[derive(Debug)]
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