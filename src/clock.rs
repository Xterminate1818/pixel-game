use std::time;

pub struct Clock {
  start: time::SystemTime,
  delta: time::Duration,
  now: time::Instant,
  fps: f64,
}

impl Clock {
  pub fn new() -> Self {
    Self {
      start: time::SystemTime::now(),
      delta: time::Duration::ZERO,
      now: time::Instant::now(),
      fps: 0.0,
    }
  }

  pub fn dt(&self) -> f64 {
    self.delta.as_secs_f64()
  }

  pub fn tick(&mut self) {
    self.delta = self.now.elapsed();
    self.now = time::Instant::now();
    self.fps = 1.0 / self.delta.as_secs_f64();
  }

  pub fn fps(&self) -> f64 {
    self.fps
  }

  pub fn since_start(&self) -> time::Duration {
    self.start.elapsed().unwrap()
  }
}
