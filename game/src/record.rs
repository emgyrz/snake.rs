use std::fs;
use std::io::Write;

static CFG_NAME: &str = "snake_rs_game_data";

pub struct Record {
  pub score: u64,
  current_score: u64,
}

impl Record {
  pub fn init() -> Self {
    let mut score = 0;
    if let Some(dir) = dirs::data_dir() {
      fs::read_to_string(dir.join(CFG_NAME))
        .iter()
        .for_each(|data| {
          data.trim().parse::<u64>().iter().for_each(|num| {
            score = *num;
          });
        });
    };
    Record {
      score,
      current_score: 0,
    }
  }

  pub fn set_current_score(&mut self, val: u64) {
    self.current_score = val;
  }

  pub fn write(&mut self) {
    if self.current_score < self.score {
      return;
    }
    self.score = self.current_score;

    if let Some(dir) = dirs::data_dir() {
      let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(dir.join(CFG_NAME));
      if let Ok(mut f) = file {
        f.write_all(self.current_score.to_string().as_bytes())
          .map_err(|e| println!("{:?}", e));
      }
    };
  }
}

impl Drop for Record {
  fn drop(&mut self) {
    self.write();
  }
}
