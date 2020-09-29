use opengl_graphics::{Texture, TextureSettings};
use snake_ctrl::{Direction, SnakeCornerVariant};
use sprite::Sprite;
use std::rc::Rc;

static STEP: f64 = 64.0;

pub struct Sprites {
  // texture: Rc<Texture>,
  apple: Sprite<Texture>,
  head_top: Sprite<Texture>,
  head_right: Sprite<Texture>,
  head_bottom: Sprite<Texture>,
  head_left: Sprite<Texture>,
  tail_top: Sprite<Texture>,
  tail_right: Sprite<Texture>,
  tail_bottom: Sprite<Texture>,
  tail_left: Sprite<Texture>,
  body_hor: Sprite<Texture>,
  body_vert: Sprite<Texture>,
  corner_top_left: Sprite<Texture>,
  corner_top_right: Sprite<Texture>,
  corner_bottom_left: Sprite<Texture>,
  corner_bottom_right: Sprite<Texture>,
}

fn take_sprite_at_pos(tex: &Rc<Texture>, x: u8, y: u8) -> Sprite<Texture> {
  let mut s = Sprite::from_texture(tex.clone());
  s.set_src_rect([f64::from(x) * STEP, f64::from(y) * STEP, STEP, STEP]);
  s.set_scale(0.25, 0.25);
  s
}

impl Sprites {
  pub fn init() -> Self {
    let img_src = include_bytes!("../../sprites.png");
    let img = image::load_from_memory(img_src).unwrap().into_rgba();
    let texture = Rc::new(Texture::from_image(&img, &TextureSettings::new()));

    let apple = take_sprite_at_pos(&texture, 0, 3);

    let head_top = take_sprite_at_pos(&texture, 3, 0);
    let head_right = take_sprite_at_pos(&texture, 4, 0);
    let head_bottom = take_sprite_at_pos(&texture, 4, 1);
    let head_left = take_sprite_at_pos(&texture, 3, 1);

    let tail_top = take_sprite_at_pos(&texture, 4, 3);
    let tail_right = take_sprite_at_pos(&texture, 3, 3);
    let tail_bottom = take_sprite_at_pos(&texture, 3, 2);
    let tail_left = take_sprite_at_pos(&texture, 4, 2);

    let body_hor = take_sprite_at_pos(&texture, 1, 0);
    let body_vert = take_sprite_at_pos(&texture, 2, 1);

    let corner_top_left = take_sprite_at_pos(&texture, 0, 0);
    let corner_top_right = take_sprite_at_pos(&texture, 2, 0);
    let corner_bottom_left = take_sprite_at_pos(&texture, 0, 1);
    let corner_bottom_right = take_sprite_at_pos(&texture, 2, 2);

    Sprites {
      // texture,
      apple,
      head_top,
      head_right,
      head_bottom,
      head_left,
      tail_top,
      tail_right,
      tail_bottom,
      tail_left,
      body_hor,
      body_vert,
      corner_top_left,
      corner_top_right,
      corner_bottom_left,
      corner_bottom_right,
    }
  }

  pub fn apple(&self) -> &Sprite<Texture> {
    &self.apple
  }

  pub fn head(&self, direction: Direction) -> &Sprite<Texture> {
    match direction {
      Direction::Top => &self.head_top,
      Direction::Right => &self.head_right,
      Direction::Bottom => &self.head_bottom,
      Direction::Left => &self.head_left,
    }
  }

  pub fn tail(&self, direction: Direction) -> &Sprite<Texture> {
    match direction {
      Direction::Top => &self.tail_top,
      Direction::Right => &self.tail_right,
      Direction::Bottom => &self.tail_bottom,
      Direction::Left => &self.tail_left,
    }
  }

  pub fn body(&self, is_vertical: bool) -> &Sprite<Texture> {
    if is_vertical {
      &self.body_vert
    } else {
      &self.body_hor
    }
  }

  pub fn corner(&self, variant: SnakeCornerVariant) -> &Sprite<Texture> {
    match variant {
      SnakeCornerVariant::TopLeft => &self.corner_top_left,
      SnakeCornerVariant::TopRight => &self.corner_top_right,
      SnakeCornerVariant::BottomLeft => &self.corner_bottom_left,
      SnakeCornerVariant::BottomRight => &self.corner_bottom_right,
    }
  }
}
