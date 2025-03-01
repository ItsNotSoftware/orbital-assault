mod entity;
mod load_level;

use entity::{Entity, EntityType};
use ggez::event;
use ggez::glam::Vec2;
use ggez::graphics::DrawParam;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{conf, Context, ContextBuilder, GameResult};
use load_level::load_level_entities;
use orbital_assault::*;

struct MainState {
    level: u8,
    running: bool,
    entities: Vec<Entity>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let entities = load_level_entities(ctx, 1);

        let s = MainState {
            level: 1,
            running: false,
            entities,
        };

        Ok(s)
    }

    fn reload_level(&mut self, ctx: &mut Context) {
        self.running = false;
        self.entities = load_level_entities(ctx, self.level);
    }

    fn next_level(&mut self, ctx: &mut Context) {
        self.level += 1;
        self.running = false;
        self.entities = load_level_entities(ctx, self.level);
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(FPS) && self.running {
            // Take missile out of the vec to not compare with itself
            let mut missile = self.entities.pop().unwrap();

            // Update missile state
            missile.apply_gravity(&self.entities, DT);
            missile.update_pos(DT);

            // Check out of bounds
            if missile.is_out_of_bounds() {
                self.reload_level(ctx);
                return Ok(());
            }

            // Colision checking
            for e in &self.entities {
                if missile.is_coliding(&e) {
                    // Check if the colision was with an UFO
                    if e.get_entity_type() == EntityType::Ufo {
                        self.next_level(ctx);
                        return Ok(());
                    } else {
                        self.reload_level(ctx);
                        return Ok(());
                    }
                }
            }

            self.entities.push(missile); // Put missile back on the Vec
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // Draw all entities
        self.entities.iter().for_each(|e| {
            e.draw(ctx, &mut canvas);
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
                self.reload_level(ctx);
            }

            Some(KeyCode::Space) => {
                self.running = true;

                // Apply force to missile
                let missile = self.entities.last_mut().unwrap();
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
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .add_resource_path("./assets");

    let (mut ctx, events_loop) = cb.build()?;
    let game = MainState::new(&mut ctx)?;

    event::run(ctx, events_loop, game)
}
