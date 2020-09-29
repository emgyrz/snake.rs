use crate::err::{SnakeCtrlErr, SnakeCtrlResult};
use crate::matrix::Matrix;
use crate::options::InnerCfg;
// use crate::utils::simple_rand;
use crate::{Direction, Point};
use rand::Rng;
use std::rc::Rc;

pub struct Food {}

impl Food {
  fn generate(cfg: &InnerCfg, snake: &[Point], food: &[Point]) -> Point {
    let max_x = cfg.dimension_x;
    let max_y = cfg.dimension_y;

    if (max_x * max_y - 1) <= (snake.len() + food.len()) as u16 {}

    let mut rng = rand::rngs::ThreadRng::default();
    let mut apple = snake[0];

    let occupied_points = [snake, food].concat();

    while occupied_points.iter().any(|p| p == &apple) {
      let x = rng.gen_range(0, max_x);
      let y = rng.gen_range(0, max_y);
      apple = Point(x, y);
    }

    apple
  }

  fn clear_eaten(food: &mut Vec<Point>, eaten: &Point) {
    if let Some(pos) = food.iter().position(|p| p == eaten) {
      food.remove(pos);
    }
  }
}

pub struct Snake {}

impl Snake {
  fn create(center: Point, size: u16) -> SnakeCtrlResult<Vec<Point>> {
    let Point(center_x, center_y) = center;

    if center_x < size {
      return Err(SnakeCtrlErr::InitSnakeSizeIsBig);
    }

    let mut snake = Vec::with_capacity(usize::from(size));
    for snake_part_ind in 0..size {
      snake.push(Point(center_x - snake_part_ind, center_y));
    }

    Ok(snake)
  }

  fn move_snake(
    cfg: &InnerCfg,
    snake: &mut Vec<Point>,
    direction: Direction,
  ) -> SnakeCtrlResult<Point> {
    let last = if let Some(l) = snake.pop() {
      l
    } else {
      return Err(SnakeCtrlErr::SnakeIsZero);
    };

    let head = if let Some(f) = snake.first() {
      f
    } else {
      return Err(SnakeCtrlErr::SnakeIsZero);
    };

    let head_x = head.0;
    let head_y = head.1;

    let new_head = match direction {
      Direction::Right => (head_x as i32 + 1, head_y as i32),
      Direction::Top => (head_x as i32, head_y as i32 + 1),
      Direction::Bottom => (head_x as i32, head_y as i32 - 1),
      Direction::Left => (head_x as i32 - 1, head_y as i32),
    };

    let new_head = Snake::try_teleport_head_if_need(cfg, new_head)?;

    snake.insert(0, new_head);

    if Snake::is_ate_itself(snake) {
      return Err(SnakeCtrlErr::SnakeAteItself);
    }

    Ok(last)
  }

  fn is_ate_itself(snake: &[Point]) -> bool {
    let head = &snake[0];
    for i in 1..snake.len() {
      let snake_part = &snake[i];
      if snake_part == head {
        return true;
      }
    }
    false
  }

  fn has_eaten(snake: &[Point], food: &[Point]) -> Option<Point> {
    let first = &snake[0];
    for f in food {
      if f == first {
        return Some(*f);
      }
    }
    None
  }

  fn try_teleport_head_if_need(
    cfg: &InnerCfg,
    new_head_unnormalized: (i32, i32),
  ) -> SnakeCtrlResult<Point> {
    let (new_head_x, new_head_y) = new_head_unnormalized;
    let can_teleport = cfg.walking_through_the_walls;
    let err =
      || -> SnakeCtrlResult<Point> { Err(SnakeCtrlErr::SnakeHitTheWall) };
    if new_head_unnormalized.0 < 0 {
      if !can_teleport {
        return err();
      }
      return Ok(Point(cfg.dimension_x - 1, new_head_y as u16));
    } else if new_head_y < 0 {
      if !can_teleport {
        return err();
      }
      return Ok(Point(new_head_x as u16, cfg.dimension_y - 1));
    } else if new_head_x > (cfg.dimension_x as i32 - 1) {
      if !can_teleport {
        return err();
      }
      return Ok(Point(0, new_head_y as u16));
    } else if new_head_unnormalized.1 > (cfg.dimension_y as i32 - 1) {
      if !can_teleport {
        return err();
      }
      return Ok(Point(new_head_x as u16, 0));
    }

    Ok(Point(new_head_x as u16, new_head_y as u16))
  }
}

pub(crate) struct Board {
  cfg: Rc<InnerCfg>,
  dim_x: u16,
  dim_y: u16,

  pub(crate) snake: Vec<Point>,
  pub(crate) food: Vec<Point>,
}
impl Board {
  pub(crate) fn new(cfg: Rc<InnerCfg>) -> SnakeCtrlResult<Self> {
    let InnerCfg {
      dimension_x,
      dimension_y,
      initial_snake_size,
      ..
    } = *cfg;

    let center = Board::center_of(dimension_x, dimension_y);
    let snake = Snake::create(center, initial_snake_size)?;

    let mut board = Board {
      cfg,
      dim_x: dimension_x,
      dim_y: dimension_y,
      snake,
      food: Vec::with_capacity(1),
    };

    if board.cfg.auto_gen_food {
      board.generate_food();
    }

    Ok(board)
  }

  pub fn restart(&mut self) -> SnakeCtrlResult<()> {
    self.snake = Snake::create(
      Board::center_of(self.cfg.dimension_x, self.cfg.dimension_y),
      self.cfg.initial_snake_size,
    )?;
    self.food = Vec::with_capacity(1);
    if self.cfg.auto_gen_food {
      self.generate_food();
    }
    Ok(())
  }

  pub(crate) fn move_snake(
    &mut self,
    direction: Direction,
  ) -> SnakeCtrlResult<bool> {
    let removed_last =
      Snake::move_snake(&self.cfg, &mut self.snake, direction)?;

    let eaten = Snake::has_eaten(&self.snake, &self.food);

    if let Some(e) = eaten {
      self.snake.push(removed_last);
      Food::clear_eaten(&mut self.food, &e);

      if self.cfg.auto_gen_food {
        self.generate_food();
      }
      return Ok(true);
    }
    return Ok(false);
  }

  pub(crate) fn generate_food(&mut self) {
    self
      .food
      .push(Food::generate(&self.cfg, &self.snake, &self.food));
  }

  fn center_of(dim_x: u16, dim_y: u16) -> Point {
    let center_x = dim_x / 2;
    let center_y = dim_y / 2;
    Point(center_x, center_y)
  }

  pub(crate) fn clone_snake(&self) -> Vec<Point> {
    self.snake.clone()
  }
  pub(crate) fn clone_food(&self) -> Vec<Point> {
    self.food.clone()
  }

  pub(crate) fn get_matrix(&self) -> Matrix {
    let mut m = Matrix::new(self.dim_x, self.dim_y);
    m.add_snake(&self.snake);
    m.add_food(&self.food);
    m
  }
}
