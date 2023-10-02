pub mod clock;
pub mod config;
pub mod draw;
pub mod engine;

use cartesian::cartesian;
use draw::SpriteBuffer;
use engine::{launch, Engine, State};
use std::collections::HashMap;

struct World {
  sprites: HashMap<(u32, u32), SpriteBuffer>,
}

impl State for World {
  fn new() -> Self {
    Self {
      sprites: draw::load_sprites(),
    }
  }
}

fn main() {
  launch(start, update, draw, end);
}

fn start(engine: &mut Engine) -> World {
  World::new()
}

fn update(state: &mut World, e: &Engine) {
  use winit::event::VirtualKeyCode as VKC;
}

fn draw(s: &World, e: &mut Engine) {
  e.clear([0x0, 0x0, 0x0, 0xFF]);
  let spr = &s.sprites[&(0, 0)];
  for (x, y) in cartesian!(0..16, 0..16) {
    e.sprite(spr, (x * 16, y * 16));
  }
}

fn end(s: &mut World, e: &mut Engine) {
}
