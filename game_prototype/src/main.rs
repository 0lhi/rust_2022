use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use ggez::event;
use ggez::graphics::{self, Color};
use ggez::timer;
//use ggez::input;
use ggez::event::quit;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::{Context, GameResult};
use glam::*;
struct MainState {
    pos_x: f32,
    speed_x: f32,
    pos_y: f32,
    speed_y: f32,
    paused: bool,
    frame_plus: bool,
    frame_minus: bool,
    frame_barrier: u8,
    cycle_count: i64,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            pos_x: 300.0,
            speed_x: SPEED,
            speed_y: -SPEED,
            pos_y: 300.0,
            paused: false,
            frame_plus: false,
            frame_minus: false,
            frame_barrier: BARRIER,
            cycle_count: 0,
        };
        Ok(s)
    }
}

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const BORDER: f32 = 10.0;
const RADIUS: f32 = 100.0;
const BARRIER: u8 = 1;
const FRAMERATE: u32 = 144;
const SPEED: f32 = 2.0;
// const MARGIN_W: f32 = 15.0;

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ggez::timer::check_update_time(ctx, FRAMERATE) {
            let mut time_direction = 1.0;
            if self.frame_minus {
                time_direction = -1.0;
            }
            if self.pos_x - RADIUS * 1.5 <= 0.0 {
                self.speed_x = self.speed_x.abs() * time_direction;
            };
            if self.pos_x + RADIUS * 1.5 >= (800.0) {
                self.speed_x = -self.speed_x.abs() * time_direction;
            };
            if self.pos_y - RADIUS * 1.5 <= 0.0 {
                self.speed_y = self.speed_y.abs() * time_direction;
            };
            if self.pos_y + RADIUS >= (600.0) {
                self.speed_y = -self.speed_y.abs() * time_direction;
            };
            if !self.paused {
                // self.pos_x = self.pos_x.rem_euclid(2000.0) + self.speed_x;
                // self.pos_y = self.pos_y.rem_euclid(2000.0) + self.speed_y;
                self.pos_x += self.speed_x;
                self.pos_y += self.speed_y;
                self.cycle_count += 1;
            }
            if self.frame_plus {
                {
                    self.frame_barrier -= 1;
                    self.pos_x += self.speed_x;
                    self.pos_y += self.speed_y;
                    self.cycle_count += 1;
                };
                if self.frame_barrier == 0 {
                    self.frame_plus = false;
                }

                // self.frame_barrier = 10
                // if self.frame_barrier >= 1 {
                //     self.pos_x += self.speed_x;
                //     self.pos_y += self.speed_y;
                // }
                // self.frame_plus = false;
            }

            if self.frame_minus {
                {
                    self.frame_barrier -= 1;
                    self.pos_x -= self.speed_x;
                    self.pos_y -= self.speed_y;
                    self.cycle_count -= 1;
                };

                if self.frame_barrier == 0 {
                    self.frame_minus = false;
                }
            }
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::P {
            self.paused = !self.paused;
        }
        if keycode == KeyCode::Plus {
            self.frame_barrier = BARRIER;
            self.frame_plus = !self.frame_plus;
        }
        if keycode == KeyCode::Minus {
            self.frame_barrier = BARRIER;
            self.frame_minus = !self.frame_minus;
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb_u32(0xff75ff));

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            RADIUS,
            0.1,
            Color::from_rgb_u32(0x007acc),
        )?;
        let circle_left = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            RADIUS / 2.0,
            0.1,
            Color::from_rgb_u32(0x007acc),
        )?;
        let circle_right = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            RADIUS / 2.0,
            0.1,
            Color::from_rgb_u32(0x007acc),
        )?;
        let pos_circle = (self.pos_x, self.pos_y);
        graphics::draw(ctx, &circle, (Vec2::new(pos_circle.0, pos_circle.1),))?;
        graphics::draw(
            ctx,
            &circle_left,
            (Vec2::new(pos_circle.0 - RADIUS, pos_circle.1 - RADIUS),),
        )?;
        graphics::draw(
            ctx,
            &circle_right,
            (Vec2::new(pos_circle.0 + RADIUS, pos_circle.1 - RADIUS),),
        )?;
        let border_collision = graphics::Rect::new(0.0, 0.0, WIDTH, HEIGHT);
        let border_drawing = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(BORDER),
            border_collision,
            Color::BLACK,
        )?;
        graphics::draw(ctx, &border_drawing, graphics::DrawParam::default())?;
        let cycles = graphics::Text::new(self.cycle_count.to_string());
        let cycle_pos = ggez::mint::Vector2 { x: 50.0, y: 550.0 };
        graphics::draw(ctx, &cycles, graphics::DrawParam::default().dest(cycle_pos))?;
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

pub fn main() -> GameResult {
    let window_cfg = WindowSetup {
        title: "Ggez Stuff".to_owned(),
        vsync: false,
        samples: NumSamples::Four,
        ..Default::default()
    };
    let window_mode = WindowMode {
        width: WIDTH,
        height: HEIGHT,
        maximized: false,
        resizable: false,
        visible: true,
        resize_on_scale_factor_change: false,
        ..Default::default()
    };
    let cb = ggez::ContextBuilder::new("super_simle", "ggez")
        .window_setup(window_cfg)
        .window_mode(window_mode);
    // .build()
    // .expect("aieee, could not create ggez context!");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
