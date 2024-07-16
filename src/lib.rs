pub use ggez::conf;
pub use ggez::event;
pub use ggez::glam::Vec2;
pub use ggez::graphics::{self, Color, Mesh};
pub use ggez::{Context, ContextBuilder, GameResult};

// Missile
pub const MISSILE_WIDTH: f32 = 42.0;
pub const MISSILE_HEIGHT: f32 = 20.0;
pub const MISSILE_MASS: f32 = 10.0;
pub const MISSILE_THRUST: f32 = 5000.0;

// UFO
pub const UFO_RADIUS: f32 = 25.0;
pub const UFO_MASS: f32 = 10.0;

// Asteroids and Planets
pub const ASTEROID_DENSITY: f32 = 100.0;
pub const PLANET_DENSITY: f32 = 10000.0;

// Game settings
pub const WINDOW_WIDTH: f32 = 890.0;
pub const WINDOW_HEIGHT: f32 = 580.0;
pub const FPS: u32 = 60;
pub const DT: f32 = 1.0 / (FPS as f32);
