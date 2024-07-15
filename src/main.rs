mod entity;
mod load_level;

use entity::Entity;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{conf, Context, ContextBuilder, GameResult};
use load_level::load_level_entities;
use orbital_assault::{WINDOW_HEIGHT, WINDOW_WIDTH};

struct MainState {
    entities: Vec<Entity>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let entities = load_level_entities(ctx);

        let s = MainState { entities };

        Ok(s)
    }

    fn reload_level(&mut self, ctx: &mut Context) {
        self.entities = load_level_entities(ctx);
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const FPS: u32 = 60;

        while ctx.time.check_update_time(FPS) {
            let _dt = ctx.time.delta();
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
                self.reload_level(ctx);
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
