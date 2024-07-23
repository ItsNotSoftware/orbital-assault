pub use ggez::conf;
pub use ggez::event;
pub use ggez::glam::Vec2;
pub use ggez::graphics::{self, Color, DrawParam, Image, Mesh};
pub use ggez::mint::Point2;
pub use ggez::{Context, ContextBuilder, GameResult};

// Missile
pub const MISSILE_WIDTH: f32 = 86.0;
pub const MISSILE_HEIGHT: f32 = 8.0;
pub const MISSILE_MASS: f32 = 10e4;
pub const MISSILE_THRUST: f32 = 1.2e9;
pub const INITIAL_SPEED: f32 = 10.0;
pub const MISSILE_IMG_OFFSET: Point2<f32> = Point2 { x: 0.465, y: 0.55 };
pub const MISSILE_IMG_SCALE: Vec2 = Vec2::new(0.2, 0.2);

// UFO
pub const UFO_RADIUS: f32 = 40.0;
pub const UFO_MASS: f32 = 10e4;
pub const UFO_IMG_OFFSET: Point2<f32> = Point2 { x: 0.5, y: 0.45 };
pub const UFO_IMG_SCALE: Vec2 = Vec2::new(0.0025, 0.0025);

// Asteroids and Planets
pub const ASTEROID_DENSITY: f32 = 100.0;
pub const PLANET_DENSITY: f32 = 1.2e11;
pub const PLANET_IMG_OFFSET: Point2<f32> = Point2 { x: 0.5, y: 0.5 };
pub const PLANET_IMG_SCALE: Vec2 = Vec2::new(0.0021, 0.0021);
pub const ASTEROID_IMG_OFFSET: Point2<f32> = Point2 { x: 0.5, y: 0.5 };
pub const ASTEROID_IMG_SCALE: Vec2 = Vec2::new(0.008, 0.008);

// Game settings
pub const WINDOW_WIDTH: f32 = 2300.0;
pub const WINDOW_HEIGHT: f32 = 1200.0;
pub const FPS: u32 = 60;
pub const DT: f32 = 1.0 / (FPS as f32);
pub const G: f32 = 6.67430 * 10e-10; // In this universe gravity is 10 times stronger
