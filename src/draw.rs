use cartesian::*;
use image::{io::Reader, ImageBuffer, Rgba};
use std::collections::HashMap;

use crate::engine::Engine;

pub type SpriteBuffer = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub fn load_sprites() -> HashMap<(u32, u32), SpriteBuffer> {
  let atlas = Reader::open("assets/sprites.png")
    .unwrap()
    .decode()
    .unwrap()
    .to_rgba8();

  let width = atlas.width() / 16;
  let height = atlas.height() / 16;
  let mut sprites = HashMap::new();

  for (sx, sy) in cartesian!(0..width, 0..height) {
    let mut copy = SpriteBuffer::new(16, 16);
    copy_sprite(&atlas, &mut copy, (sx * 16, sy * 16, 16, 16), (0, 0));
    sprites.insert((sx, sy), copy);
  }

  sprites
}

pub fn copy_sprite(
  from: &SpriteBuffer,
  to: &mut SpriteBuffer,
  bounds: (u32, u32, u32, u32),
  target: (u32, u32),
) {
  let (left, top, width, height) = bounds;
  for (x, y) in cartesian!(left..left + width, top..top + height) {
    let pixel_copy = match from.get_pixel_checked(x, y) {
      Some(pixel) => pixel,
      None => continue,
    };
    match to.get_pixel_mut_checked(x + target.0, y + target.1) {
      Some(pixel_target) => *pixel_target = *pixel_copy,
      None => continue,
    }
  }
}

pub fn copy_entire_sprite(
  from: &SpriteBuffer,
  to: &mut SpriteBuffer,
  target: (u32, u32),
) {
  copy_sprite(from, to, (0, 0, from.width(), from.height()), target);
}

impl Engine {
  #[inline]
  pub fn world_to_screen(&self, world: (i32, i32)) -> Option<(u32, u32)> {
    let x = world.0 + self.eye.0;
    let y = world.1 + self.eye.1;
    match (x.try_into(), y.try_into()) {
      (Ok(x), Ok(y)) => Some((x, y)),
      _ => None,
    }
  }

  #[inline]
  pub fn pixel_direct(&mut self, pixel: [u8; 4], pos: (u32, u32)) {
    let index = (pos.0 + pos.1 * Self::WIDTH) as usize;
    if let Some(p) = self.fb.frame_mut().chunks_exact_mut(4).nth(index) {
      p.copy_from_slice(&pixel)
    }
  }

  #[inline]
  pub fn pixel(&mut self, pixel: [u8; 4], pos: (i32, i32)) {
    let pos = match self.world_to_screen(pos) {
      Some(pos) => pos,
      None => return,
    };
    self.pixel_direct(pixel, pos);
  }

  #[inline]
  pub fn sprite_direct(&mut self, sprite: &SpriteBuffer, target: (u32, u32)) {
    for (x, y) in cartesian!(0..sprite.width(), 0..sprite.height()) {
      match sprite.get_pixel_checked(x, y) {
        Some(pixel) => self.pixel_direct(pixel.0, (x + target.0, y + target.1)),
        None => continue,
      }
    }
  }

  #[inline]
  pub fn sprite(&mut self, sprite: &SpriteBuffer, target: (i32, i32)) {
    for (x, y) in cartesian!(0..sprite.width(), 0..sprite.height()) {
      match sprite.get_pixel_checked(x, y) {
        Some(pixel) => {
          self.pixel(pixel.0, (x as i32 + target.0, y as i32 + target.1))
        },
        None => continue,
      }
    }
  }

  #[inline]
  pub fn clear(&mut self, color: [u8; 4]) {
    self
      .fb
      .frame_mut()
      .chunks_exact_mut(4)
      .for_each(|px| px.copy_from_slice(&color))
  }

  #[inline]
  pub fn look(&mut self, pos: (i32, i32)) {
    self.eye = (
      pos.0 + (Self::WIDTH / 2) as i32,
      pos.1 + (Self::HEIGHT / 2) as i32,
    )
  }
}
