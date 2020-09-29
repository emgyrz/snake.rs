use crate::Point;
use std::fmt;

pub struct Matrix {
  pub rows: Vec<Vec<u8>>,
}

impl Matrix {
  pub fn new(dim_x: u16, dim_y: u16) -> Self {
    let mut rows = Vec::with_capacity(dim_y as usize);
    for _ in 0..dim_y {
      let mut cells = Vec::with_capacity(dim_x as usize);
      for _ in 0..dim_x {
        cells.push(0);
      }
      rows.push(cells)
    }

    Matrix { rows }
  }

  pub(crate) fn add_snake(&mut self, snake: &[Point]) {
    for point in snake {
      self.rows[point.1 as usize][point.0 as usize] = 1;
    }
  }

  pub(crate) fn add_food(&mut self, food: &[Point]) {
    for f in food {
      self.rows[f.1 as usize][f.0 as usize] = 7;
    }
  }
}

impl fmt::Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let result = self
      .rows
      .iter()
      .map(|cells| {
        cells
          .iter()
          .map(|s| s.to_string())
          .collect::<Vec<String>>()
          .join(" ")
      })
      .collect::<Vec<String>>()
      .join("\n");
    write!(f, "{}", result)
  }
}
