use crate::{Direction, Point};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SnakeCornerVariant {
  TopLeft,
  TopRight,
  BottomLeft,
  BottomRight,
}

impl SnakeCornerVariant {
  fn reverse_y(&self) -> Self {
    match self {
      SnakeCornerVariant::TopLeft => SnakeCornerVariant::BottomLeft,
      SnakeCornerVariant::TopRight => SnakeCornerVariant::BottomRight,
      SnakeCornerVariant::BottomLeft => SnakeCornerVariant::TopLeft,
      SnakeCornerVariant::BottomRight => SnakeCornerVariant::TopRight,
    }
  }
}

pub enum SnakePartVariant {
  Head(Direction),
  Tail(Direction),
  Body(bool),
  Corner(SnakeCornerVariant),
}

pub struct SnakePart {
  pub point: Point,
  pub variant: SnakePartVariant,
}

impl SnakePart {
  fn new(p: Point, v: SnakePartVariant) -> SnakePart {
    SnakePart {
      point: p,
      variant: v,
    }
  }
}

pub struct SnakeCtrlFullState {
  pub snake: Vec<SnakePart>,
  pub food: Vec<Point>,
  pub direction: Direction,
}

pub(crate) fn calc_full_state(
  snake: &[Point],
  food: &[Point],
  current_direction: Direction,
  dim_y: u16,
  reverse_y: bool,
) -> SnakeCtrlFullState {
  let snake_len = snake.len();
  let max_ind = snake.len() - 1;
  let mut result: Vec<SnakePart> = Vec::with_capacity(snake_len);

  for (ind, curr_point) in snake.iter().enumerate() {
    if ind == 0 {
      result.push(SnakePart::new(
        *curr_point,
        SnakePartVariant::Head(current_direction),
      ));
    } else if ind == max_ind {
      let pre_tail = snake.get(max_ind - 1).unwrap();
      let mut tail_direction = curr_point.offset_from_near(&pre_tail).unwrap();
      if !pre_tail.is_near_with(&curr_point) {
        tail_direction = tail_direction.opposite_direction();
      }
      result.push(SnakePart::new(
        *curr_point,
        SnakePartVariant::Tail(tail_direction),
      ))
    } else {
      let prev = snake.get(ind + 1).unwrap();
      let next = snake.get(ind - 1).unwrap();
      result.push(SnakePart::new(
        *curr_point,
        get_body_part_variant(prev, curr_point, next),
      ))
    }
  }

  let mut f = food.to_owned();
  f.iter_mut().for_each(|p| p.reverse_y(dim_y));

  if reverse_y {
    result.iter_mut().for_each(|p| {
      p.point.reverse_y(dim_y);

      if let SnakePartVariant::Corner(v) = p.variant {
        p.variant = SnakePartVariant::Corner(v.reverse_y());
      }
    });
  }

  SnakeCtrlFullState {
    snake: result,
    food: f,
    direction: current_direction,
  }
}

fn get_body_part_variant(
  prev: &Point,
  curr: &Point,
  next: &Point,
) -> SnakePartVariant {
  let mut offset_from_prev = curr.offset_from_near(prev).unwrap();
  let mut offset_from_next = next.offset_from_near(curr).unwrap();

  if offset_from_prev == offset_from_next
    || offset_from_prev == offset_from_next.opposite_direction()
  {
    return SnakePartVariant::Body(offset_from_prev.is_vertical());
  }

  if !next.is_near_with(curr) {
    offset_from_next = offset_from_next.opposite_direction();
  }
  if !prev.is_near_with(curr) {
    offset_from_prev = offset_from_prev.opposite_direction();
  }

  match offset_from_prev {
    Direction::Top => {
      if offset_from_next == Direction::Right {
        SnakePartVariant::Corner(SnakeCornerVariant::BottomLeft)
      } else {
        SnakePartVariant::Corner(SnakeCornerVariant::BottomRight)
      }
    }
    Direction::Bottom => {
      if offset_from_next == Direction::Right {
        SnakePartVariant::Corner(SnakeCornerVariant::TopLeft)
      } else {
        SnakePartVariant::Corner(SnakeCornerVariant::TopRight)
      }
    }
    Direction::Left => {
      if offset_from_next == Direction::Top {
        SnakePartVariant::Corner(SnakeCornerVariant::TopLeft)
      } else {
        SnakePartVariant::Corner(SnakeCornerVariant::BottomLeft)
      }
    }
    Direction::Right => {
      if offset_from_next == Direction::Top {
        SnakePartVariant::Corner(SnakeCornerVariant::TopRight)
      } else {
        SnakePartVariant::Corner(SnakeCornerVariant::BottomRight)
      }
    }
  }
}
