use thiserror::Error;

#[derive(Error, Debug)]
pub enum SnakeCtrlErr {
  #[error("row index ({0}) is out of bounds")]
  RowIndexOutOfBounds(u16),
  #[error("column index ({0}) is out of bounds")]
  ColumnIndexOutOfBounds(u16),
  #[error("the snake ate itself")]
  SnakeAteItself,
  #[error("the snake hit the wall")]
  SnakeHitTheWall,
  #[error("initial snake size is more than possible")]
  InitSnakeSizeIsBig,
  #[error("something is really wrong. your snake size is zero")]
  SnakeIsZero,
}

pub type SnakeCtrlResult<T> = Result<T, SnakeCtrlErr>;
