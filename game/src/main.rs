#![windows_subsystem = "windows"]
pub mod consts;
mod record;
mod timer;
mod view;

#[macro_use]
extern crate lazy_static;

use glutin_window::GlutinWindow as Window;

static BOARD_DIM_X: u16 = 27;
static BOARD_DIM_Y: u16 = 23;

use consts::{HALF_STEP, STEP};
use graphics::rectangle::Border;
use graphics::{text, DrawState, Rectangle, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;
use snake_ctrl::{Direction, SnakeCtrl, SnakeCtrlErr, SnakeCtrlOptions};
use view::colors;

pub struct App<'a> {
  gl: GlGraphics,
  snake_ctrl: SnakeCtrl,
  score: u64,
  glyph_cache: GlyphCache<'a>,
  sprites: view::Sprites,
  timer: timer::Timer,
  record: record::Record,
  is_game_over: bool,
  def_draw_state: DrawState,
}

impl<'a> App<'a> {
  fn handle_key_press(&mut self, key: Key) {
    match key {
      Key::Right => {
        self.snake_ctrl.direction_to(Direction::Right).unwrap();
      }
      Key::Left => {
        self.snake_ctrl.direction_to(Direction::Left).unwrap();
      }
      Key::Up => {
        self.snake_ctrl.direction_to(Direction::Top).unwrap();
      }
      Key::Down => {
        self.snake_ctrl.direction_to(Direction::Bottom).unwrap();
      }
      Key::Space => {
        if self.is_game_over {
          self.restart_game();
        } else {
          self.timer.toggle_pause();
        }
      }
      _ => {}
    }
  }

  fn restart_game(&mut self) {
    self.score = 0;
    self.timer = timer::Timer::new(200);
    self.snake_ctrl.restart().unwrap();
    self.is_game_over = false;
  }

  fn render(&mut self, args: &RenderArgs) {
    let viewport = args.viewport();
    let win_width = viewport.window_size[0];
    let win_height = viewport.window_size[1];
    let glyphs = &mut self.glyph_cache;
    let def_draw_state = &self.def_draw_state;
    let score = self.score;
    let is_record = self.record.score < score;

    self.gl.draw(viewport, |c, gl| {
      graphics::clear(*colors::BLACK, gl);
      text::Text::new_color(*colors::WHITE, 32)
        .draw(
          &format!("Score: {}", score),
          glyphs,
          def_draw_state,
          c.transform.trans(30.0, 40.0).zoom(0.5),
          gl,
        )
        .unwrap();

      if is_record {
        text::Text::new_color(*colors::ORANGE, 32)
          .draw(
            "Wow! It's a record!",
            glyphs,
            def_draw_state,
            c.transform.trans(100.0, 40.0).zoom(0.5),
            gl,
          )
          .unwrap();
      }
    });

    let offset = (50.0, 70.0);
    let state = self.snake_ctrl.get_full_state_reversed_y();

    let food_sprite = self.sprites.apple();

    self.gl.draw(viewport, |c, gl| {
      let r = Rectangle::new(*colors::LIME).border(Border {
        color: *colors::DARK_GREEN,
        radius: 1.0,
      });
      r.draw(
        [
          0.0,
          0.0,
          STEP * (BOARD_DIM_X as f64) + 2.0,
          STEP * (BOARD_DIM_Y as f64) + 2.0,
        ],
        def_draw_state,
        c.transform.trans(offset.0 - STEP - 1.0, offset.1 - 1.0),
        gl,
      );
      for food in &state.food {
        let x = food.0 as f64 * STEP - HALF_STEP + offset.0;
        let y = food.1 as f64 * STEP - HALF_STEP + offset.1;
        food_sprite.draw(c.transform.trans(x, y), gl);
      }
    });

    let speed_ms = self.timer.get_speed();
    self.gl.draw(viewport, |c, gl| {
      let r = Rectangle::new(*colors::GREY);
      r.draw(
        [
          0.0,
          win_height - 20.0,
          win_width,
          win_height,
        ],
        def_draw_state,
        c.transform.trans(0.0, 0.0),
        gl,
      );

      text::Text::new_color(*colors::WHITE, 24)
        .draw(
          &format!(
            r#"Esc: Quit      Space: Pause/Resume      Size: {}x{}      Speed: {}ms"#,
            BOARD_DIM_X, BOARD_DIM_Y, speed_ms
          ),
          glyphs,
          def_draw_state,
          c.transform.trans(30.0, win_height - 6.0).zoom(0.5),
          gl,
        )
        .unwrap();
    });

    view::draw_snake(
      &mut self.gl,
      args.viewport(),
      &state,
      &self.sprites,
      offset,
    );

    if self.is_game_over {
      self.gl.draw(viewport, |c, gl| {
        let r = Rectangle::new(*colors::BLACK_OP);
        r.draw(
          [0.0, 0.0, win_width, win_height],
          def_draw_state,
          c.transform.trans(0.0, 0.0),
          gl,
        );

        text::Text::new_color(*colors::WHITE, 64)
          .draw(
            "Game over :(",
            glyphs,
            def_draw_state,
            c.transform
              .trans(win_width / 2.0 - 90.0, win_height / 2.0 - 15.0)
              .zoom(0.5),
            gl,
          )
          .unwrap();

        text::Text::new_color(*colors::WHITE, 40)
          .draw(
            "Press Space to restart",
            glyphs,
            def_draw_state,
            c.transform
              .trans(win_width / 2.0 - 100.0, win_height / 2.0 + 10.0)
              .zoom(0.5),
            gl,
          )
          .unwrap();
      });
    }
  }

  fn update(&mut self, _args: &UpdateArgs) {
    match self.snake_ctrl.next_tick() {
      Ok(is_ate) => {
        if is_ate {
          self.score += 1;
          self.record.set_current_score(self.score);
          self.timer.decrease_tick_millis();
        }
      }
      Err(e) => match e {
        SnakeCtrlErr::SnakeAteItself | SnakeCtrlErr::SnakeHitTheWall => {
          self.is_game_over = true;
          self.record.write();
          self.timer.pause();
        }
        _ => eprintln!("{:?}", e),
      },
    }
  }
}

fn main() {
  let opengl = OpenGL::V3_2;

  let mut window: Window = WindowSettings::new(
    "(Snake game).rs",
    [
      (BOARD_DIM_X * STEP as u16 + 70) as u32,
      (BOARD_DIM_Y * STEP as u16 + 130) as u32,
    ],
  )
  .resizable(false)
  .graphics_api(opengl)
  .exit_on_esc(true)
  .build()
  .unwrap();

  let snake_ctrl_options = SnakeCtrlOptions::default()
    .dimension_x(BOARD_DIM_X)
    .dimension_y(BOARD_DIM_Y)
    .initial_snake_size(10);

  let glyph_cache = GlyphCache::from_bytes(
    include_bytes!("../FiraSans-Regular.ttf"),
    (),
    TextureSettings::new(),
  )
  .unwrap();

  let mut app = App {
    gl: GlGraphics::new(opengl),
    score: 0,
    snake_ctrl: SnakeCtrl::new(&snake_ctrl_options).unwrap(),
    glyph_cache,
    sprites: view::Sprites::init(),
    timer: timer::Timer::new(150),
    record: record::Record::init(),
    is_game_over: false,
    def_draw_state: DrawState::default(),
  };

  let mut events = Events::new(EventSettings::new().max_fps(30));

  while let Some(e) = events.next(&mut window) {
    if let Some(args) = e.render_args() {
      app.render(&args);
    }

    if let Some(Button::Keyboard(key)) = e.press_args() {
      app.handle_key_press(key);
    };

    if let Some(args) = e.update_args() {
      if app.timer.is_ready() {
        app.update(&args);
      }
    }
  }
}
