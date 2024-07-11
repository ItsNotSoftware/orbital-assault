pub use ggez::conf;
pub use ggez::event;
pub use ggez::glam::Vec2;
pub use ggez::graphics::{self, Color, Mesh};
pub use ggez::{Context, ContextBuilder, GameResult};

// Missile
pub const MISSILE_RADIUS: f32 = 10.0;
pub const MISSILE_MASS: f32 = 10.0;

// UFO
pub const UFO_RADIUS: f32 = 10.0;
pub const UFO_MASS: f32 = 10.0;

// Asteroids and Planets
pub const ASTEROID_DENSITY: f32 = 100.0;
pub const PLANET_DENSITY: f32 = 1000.0;

// Window
pub const WINDOW_WIDTH: f32 = 890.0;
pub const WINDOW_HEIGHT: f32 = 580.0;
