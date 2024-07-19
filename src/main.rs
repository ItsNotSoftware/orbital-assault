mod engine;
mod entity;
mod load_level;

use entity::Entity;
use ggez::event;
use ggez::glam::Vec2;
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{conf, Context, ContextBuilder, GameResult};
use load_level::load_level_entities;
use orbital_assault::*;

struct MainState {
    running: bool,
    entities: Vec<Entity>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let entities = load_level_entities(ctx);

        let s = MainState {
            running: false,
            entities,
        };

        Ok(s)
    }

    fn reload_level(&mut self, ctx: &mut Context) {
        self.entities = load_level_entities(ctx);
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const MISSILE_IDX: usize = 0;

        while ctx.time.check_update_time(FPS) && self.running {
            // Update missile state
            let mut missile = self.entities[MISSILE_IDX].clone(); // Borrow checker workaround ¯\_(ツ)_/¯
            missile.apply_gravity(&self.entities, DT, MISSILE_IDX);
            missile.update_pos(DT);
            self.entities[MISSILE_IDX] = missile;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // Draw all entities
        self.entities.iter().for_each(|e| {
            e.draw(&mut canvas);
        });

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        match input.keycode {
            Some(KeyCode::R) => {
                self.running = false;
                self.reload_level(ctx);
            }

            Some(KeyCode::Space) => {
                self.running = true;

                // Apply force to missile
                let missile = &mut self.entities[0];
                let (_, angle) = missile.get_pose();

                let force = Vec2::new(MISSILE_THRUST * angle.cos(), MISSILE_THRUST * angle.sin());
                missile.apply_force(force, ctx.time.delta().as_secs_f32());
            }

            Some(KeyCode::Escape) => ctx.request_quit(),

            _ => (),
        }

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ContextBuilder::new("orbital-assault", "ItsNotSoftware")
        .window_setup(conf::WindowSetup::default().title("Orbital Assault!"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));

    let (mut ctx, events_loop) = cb.build()?;
    let game = MainState::new(&mut ctx)?;

    event::run(ctx, events_loop, game)
}
