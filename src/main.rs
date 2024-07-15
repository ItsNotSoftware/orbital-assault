mod entity;
mod load_level;

use entity::Entity;
use orbital_assault::*;

struct MainState {
    entities: Vec<Entity>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let planet = entity::create_planet(ctx, 400.0, 300.0, 50.0);

        let s = MainState {
            entities: vec![planet],
        };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const FPS: u32 = 60;

        while ctx.time.check_update_time(FPS) {
            let dt = ctx.time.delta();
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
}

pub fn main() -> GameResult {
    let cb = ContextBuilder::new("orbital-assault", "ItsNotSoftware")
        .window_setup(conf::WindowSetup::default().title("Orbital Assault!"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));

    let (mut ctx, events_loop) = cb.build()?;
    let game = MainState::new(&mut ctx)?;

    event::run(ctx, events_loop, game)
}
