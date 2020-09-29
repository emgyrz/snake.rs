mod board;
mod err;
mod full_state;
mod matrix;
mod options;

use crate::full_state::calc_full_state;
pub use crate::full_state::{
  SnakeCornerVariant, SnakeCtrlFullState, SnakePart, SnakePartVariant,
};
pub use crate::matrix::Matrix as SnakeCtrlMatrix;
use crate::options::InnerCfg;
use board::Board;
pub use err::{SnakeCtrlErr, SnakeCtrlResult};
pub use options::Options as SnakeCtrlOptions;
use std::rc::Rc;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction {
  Top,
  Right,
  Bottom,
  Left,
}

impl Direction {
  pub fn opposite_direction(&self) -> Self {
    match self {
      Direction::Top => Direction::Bottom,
      Direction::Right => Direction::Left,
      Direction::Bottom => Direction::Top,
      Direction::Left => Direction::Right,
    }
  }

  pub fn is_vertical(&self) -> bool {
    match self {
      Direction::Top | Direction::Bottom => true,
      Direction::Left | Direction::Right => false,
    }
  }
  pub fn is_horizontal(&self) -> bool {
    !self.is_vertical()
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Point(pub u16, pub u16);
impl Point {
  pub fn is_near_with(&self, p: &Point) -> bool {
    if self.0 == p.0 {
      (self.1 as i32 - p.1 as i32).abs() == 1
    } else {
      (self.0 as i32 - p.0 as i32).abs() == 1
    }
  }

  pub fn offset_from_near(&self, p: &Point) -> Option<Direction> {
    if self.0 == p.0 {
      return Some(if self.1 < p.1 {
        Direction::Bottom
      } else {
        Direction::Top
      });
    } else if self.1 == p.1 {
      return Some(if self.0 < p.0 {
        Direction::Left
      } else {
        Direction::Right
      });
    }

    None
  }

  pub fn reverse_y(&mut self, dimension_y: u16) {
    self.1 = dimension_y - self.1;
  }
}

pub struct SnakeCtrl {
  next_direction: Direction,
  current_direction: Direction,
  board: Board,
  cfg: Rc<InnerCfg>,
}

pub struct SnakeCtrlState {
  pub snake: Vec<Point>,
  pub food: Vec<Point>,
  pub head_direction: Direction,
  pub tail_direction: Direction,
}

impl SnakeCtrl {
  pub fn new(opts: &SnakeCtrlOptions) -> SnakeCtrlResult<Self> {
    let cfg = Rc::new(InnerCfg::from_options(opts));
    let board = Board::new(cfg.clone())?;

    Ok(SnakeCtrl {
      next_direction: Direction::Right,
      board,
      cfg,
      current_direction: Direction::Right,
    })
  }

  pub fn direction_to(&mut self, direction: Direction) -> SnakeCtrlResult<()> {
    let is_opposite_direction =
      self.current_direction.opposite_direction() == direction;
    if self.cfg.fail_on_revert && is_opposite_direction {
      return Err(SnakeCtrlErr::SnakeAteItself);
    }

    if !is_opposite_direction {
      self.next_direction = direction;
    }

    Ok(())
  }

  pub fn current_direction(&self) -> Direction {
    self.current_direction
  }

  pub fn next_tick(&mut self) -> SnakeCtrlResult<bool> {
    self.current_direction = self.next_direction;
    self.board.move_snake(self.next_direction)
  }

  pub fn restart(&mut self) -> SnakeCtrlResult<()> {
    self.next_direction = Direction::Right;
    self.current_direction = Direction::Right;
    self.board.restart()
  }

  pub fn get_state(&self) -> SnakeCtrlState {
    let snake = self.board.clone_snake();
    let food = self.board.clone_food();
    let head_direction = self.current_direction;
    let tail = snake.last().unwrap();
    let pre_tail = snake.get(snake.len() - 2).unwrap();
    let tail_direction = tail.offset_from_near(pre_tail).unwrap();

    SnakeCtrlState {
      snake,
      food,
      head_direction,
      tail_direction,
    }
  }

  pub fn get_full_state(&self) -> SnakeCtrlFullState {
    calc_full_state(
      &self.board.snake,
      &self.board.food,
      self.current_direction,
      self.cfg.dimension_y,
      false,
    )
  }
  pub fn get_full_state_reversed_y(&self) -> SnakeCtrlFullState {
    calc_full_state(
      &self.board.snake,
      &self.board.food,
      self.current_direction,
      self.cfg.dimension_y,
      true,
    )
  }

  pub fn get_state_reversed_y(&self) -> SnakeCtrlState {
    let dim_y = self.cfg.dimension_y;
    let mut state = self.get_state();
    state.snake.iter_mut().for_each(|p| p.reverse_y(dim_y));
    state.food.iter_mut().for_each(|p| p.reverse_y(dim_y));
    state
  }

  pub fn get_matrix(&self) -> SnakeCtrlMatrix {
    self.board.get_matrix()
  }
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn tst() {
    let opts = SnakeCtrlOptions::default()
      .dimension_x(15)
      .dimension_y(10)
      .initial_snake_size(7);
    let mut ctrl = SnakeCtrl::new(&opts).unwrap();
    println!("{}\n", ctrl.get_matrix());
    ctrl.direction_to(Direction::Left).unwrap();
    ctrl.next_tick();
    println!("{}\n", ctrl.get_matrix());
    ctrl.direction_to(Direction::Left).unwrap();
    ctrl.next_tick();
    println!("{}\n", ctrl.get_matrix());
    ctrl.direction_to(Direction::Top).unwrap();
    ctrl.next_tick();
    println!("{}\n", ctrl.get_matrix());
    // ctrl.direction_to(Direction::Right);
    ctrl.next_tick();
    println!("{}\n", ctrl.get_matrix());
    ctrl.direction_to(Direction::Right).unwrap();

    ctrl.next_tick();
    println!("{}\n", ctrl.get_matrix());
    ctrl.direction_to(Direction::Left).unwrap();
    ctrl.next_tick();
    println!("{}\n", ctrl.get_matrix());
  }
}
