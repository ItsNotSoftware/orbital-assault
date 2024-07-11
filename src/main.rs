mod entity;

use entity::Entity;
use orbital_assault::*;

struct MainState {
    entities: Vec<Entity>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let planet = entity::create_planet(400.0, 300.0, 50.0);

        let s = MainState {
            entities: vec![planet],
        };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // Draw all entities
        self.entities.iter().for_each(|e| {
            canvas.draw(&e.get_mesh(ctx), e.get_pos());
        });

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ContextBuilder::new("orbital-assault", "ItsNotSoftware")
        .window_setup(conf::WindowSetup::default().title("Orbital Assault!"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));

    let (ctx, events_loop) = cb.build()?;
    let game = MainState::new()?;

    event::run(ctx, events_loop, game)
}
