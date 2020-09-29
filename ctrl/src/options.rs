static DEF_DIMENSION_X: u16 = 7;
static DEF_DIMENSION_Y: u16 = 7;
static DEF_INITIAL_SNAKE_SIZE: u16 = 3;
static DEF_WALKING_THROUGH_THE_WALLS: bool = true;
static DEF_FAIL_ON_REVERT: bool = false;
static DEF_AUTO_GEN_FOOD: bool = true;

#[derive(Default)]
pub struct Options {
  dimension_x: Option<u16>,
  dimension_y: Option<u16>,
  initial_snake_size: Option<u16>,
  walking_through_the_walls: Option<bool>,
  fail_on_revert: Option<bool>,
  auto_gen_food: Option<bool>,
}

impl Options {
  pub fn dimension_x(mut self, val: u16) -> Self {
    self.dimension_x = Some(val);
    self
  }
  pub fn dimension_y(mut self, val: u16) -> Self {
    self.dimension_y = Some(val);
    self
  }
  pub fn initial_snake_size(mut self, val: u16) -> Self {
    self.initial_snake_size = Some(val);
    self
  }
  pub fn walking_through_the_walls(mut self, val: bool) -> Self {
    self.walking_through_the_walls = Some(val);
    self
  }
  pub fn fail_on_revert(mut self, val: bool) -> Self {
    self.fail_on_revert = Some(val);
    self
  }
  pub fn auto_gen_food(mut self, val: bool) -> Self {
    self.auto_gen_food = Some(val);
    self
  }
}

pub(crate) struct InnerCfg {
  pub(crate) dimension_x: u16,
  pub(crate) dimension_y: u16,
  pub(crate) initial_snake_size: u16,
  pub(crate) walking_through_the_walls: bool,
  pub(crate) fail_on_revert: bool,
  pub(crate) auto_gen_food: bool,
}

impl InnerCfg {
  pub(crate) fn from_options(opts: &Options) -> Self {
    let snake_size = if let Some(s) = opts.initial_snake_size {
      if s < 3 {
        3
      } else {
        s
      }
    } else {
      DEF_INITIAL_SNAKE_SIZE
    };

    InnerCfg {
      dimension_x: opts.dimension_x.unwrap_or(DEF_DIMENSION_X),
      dimension_y: opts.dimension_y.unwrap_or(DEF_DIMENSION_Y),
      initial_snake_size: snake_size,
      walking_through_the_walls: opts
        .walking_through_the_walls
        .unwrap_or(DEF_WALKING_THROUGH_THE_WALLS),
      fail_on_revert: opts.fail_on_revert.unwrap_or(DEF_FAIL_ON_REVERT),
      auto_gen_food: opts.auto_gen_food.unwrap_or(DEF_AUTO_GEN_FOOD),
    }
  }
}
