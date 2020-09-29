use crate::view::Sprites;
use graphics::{Transformed, Viewport};
use opengl_graphics::GlGraphics;
use snake_ctrl::{SnakeCtrlFullState, SnakePartVariant};

use crate::consts::{HALF_STEP, STEP};

pub fn draw_snake(
  gl: &mut GlGraphics,
  vp: Viewport,
  state: &SnakeCtrlFullState,
  sprites: &Sprites,
  offset: (f64, f64),
) {
  gl.draw(vp, |c, gl| {
    for snake_part in &state.snake {
      let x = snake_part.point.0 as f64 * STEP - HALF_STEP + offset.0;
      let y = snake_part.point.1 as f64 * STEP - HALF_STEP + offset.1;
      let transform = c.transform.trans(x, y);

      let sprite = match snake_part.variant {
        SnakePartVariant::Head(dir) => sprites.head(dir),
        SnakePartVariant::Tail(dir) => sprites.tail(dir),
        SnakePartVariant::Body(is_vertical) => sprites.body(is_vertical),
        SnakePartVariant::Corner(var) => sprites.corner(var),
      };

      sprite.draw(transform, gl);
    }
  });
}
