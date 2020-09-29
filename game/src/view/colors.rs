use colorsys::Rgb;

fn from_rgb_to_ratio<T: Into<Rgb>>(val: T) -> [f32; 4] {
  let rgb: Rgb = val.into();
  rgb.as_ratio().into()
}

lazy_static! {
  pub static ref WHITE: [f32; 4] = from_rgb_to_ratio((255, 255, 255));
  pub static ref LIME: [f32; 4] = from_rgb_to_ratio((205, 220, 57));
  pub static ref DARK_GREEN: [f32; 4] = from_rgb_to_ratio((130, 119, 23));
  pub static ref BLACK: [f32; 4] = from_rgb_to_ratio((33, 33, 33));
  pub static ref BLACK_OP: [f32; 4] =
    from_rgb_to_ratio((33.0, 33.0, 33.0, 0.9));
  pub static ref ORANGE: [f32; 4] = from_rgb_to_ratio((224, 93, 31));
  pub static ref GREY: [f32; 4] = from_rgb_to_ratio((50, 50, 50));
}
