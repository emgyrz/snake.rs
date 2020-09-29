use std::time::Instant;

pub struct Timer {
  start: Instant,
  last_update: Instant,
  tick_millis: u128,
  count_of_decreasing: u32,
  is_paused: bool,
}

impl Timer {
  pub fn new(tick_millis: u128) -> Self {
    Timer {
      last_update: Instant::now(),
      start: Instant::now(),
      tick_millis,
      count_of_decreasing: 0,
      is_paused: false,
    }
  }

  pub fn is_ready(&mut self) -> bool {
    if !self.is_paused
      && (self.last_update.elapsed().as_millis() >= self.tick_millis)
    {
      self.last_update = Instant::now();
      return true;
    }
    return false;
  }

  pub fn decrease_tick_millis(&mut self) {
    self.count_of_decreasing += 1;
    self.tick_millis = ((self.tick_millis as f64) * 0.99999) as u128;
  }

  pub fn pause(&mut self) {
    self.is_paused = true;
  }

  pub fn resume(&mut self) {
    self.is_paused = false;
  }

  pub fn toggle_pause(&mut self) {
    if self.is_paused {
      self.resume()
    } else {
      self.pause()
    }
  }

  pub fn get_speed(&self) -> u128 {
    self.tick_millis
  }

  pub fn reset_last_update(&mut self) {
    self.last_update = self.start;
  }
}
