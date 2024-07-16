use graphics::Canvas;
use orbital_assault::*;

#[derive(Debug, Clone, Copy)]
pub enum EntityType {
    Missile,
    Planet,
    Asteroid,
    Ufo,
}

#[derive(Debug, Clone)]
pub struct Entity {
    e_type: EntityType,
    pos: Vec2,
    vel: Vec2,  // Bodyframe velocity
    angle: f32, // Angle in respect to the x-axis
    ang_vel: f32,
    mass: f32,
    mesh: Mesh,
}
impl Entity {
    pub fn get_pose(&self) -> (Vec2, f32) {
        (self.pos, self.angle)
    }

    pub fn get_mass(&self) -> f32 {
        self.mass
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.draw(&self.mesh, self.pos);
    }

    pub fn update_pos(&mut self, dt: f32) {
        dbg!(self.vel);
        self.pos.x += self.vel.x * dt;
        self.pos.y += self.vel.y * dt;
    }

    pub fn apply_force(&mut self, force: Vec2, dt: f32) {
        self.vel.x += force.x / self.mass * dt;
        self.vel.y += force.y / self.mass * dt;
    }
}

pub fn create_missile(ctx: &mut Context, x: f32, y: f32, angle: f32) -> Entity {
    let mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0, 0.0, MISSILE_WIDTH, MISSILE_HEIGHT),
        Color::RED,
    )
    .expect("Could not create mesh");

    Entity {
        e_type: EntityType::Missile,
        pos: Vec2::new(x, y),
        vel: Vec2::new(0.0, 0.0),
        angle,
        ang_vel: 0.0,
        mass: MISSILE_MASS,
        mesh,
    }
}

pub fn create_planet(ctx: &mut Context, x: f32, y: f32, radius: f32) -> Entity {
    let mesh = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        Vec2::new(0.0, 0.0),
        radius,
        2.0,
        Color::MAGENTA,
    )
    .expect("Could not create mesh");

    Entity {
        e_type: EntityType::Planet,
        pos: Vec2::new(x, y),
        vel: Vec2::new(0.0, 0.0),
        angle: 0.0,
        ang_vel: 0.0,
        mass: PLANET_DENSITY * radius * radius * std::f32::consts::PI,
        mesh,
    }
}

pub fn create_asteroid(
    ctx: &mut Context,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radious: f32,
) -> Entity {
    let mesh = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        Vec2::new(0.0, 0.0),
        radious,
        2.0,
        Color::BLACK,
    )
    .expect("Could not create mesh");

    Entity {
        e_type: EntityType::Asteroid,
        pos: Vec2::new(x, y),
        vel: Vec2::new(vx, vy),
        angle: 0.0,
        ang_vel: 0.0,
        mass: ASTEROID_DENSITY * radious * radious * std::f32::consts::PI,
        mesh,
    }
}

pub fn create_ufo(ctx: &mut Context, x: f32, y: f32, vx: f32, vy: f32) -> Entity {
    let angle = (vy / vx).atan();

    let mesh = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        Vec2::new(0.0, 0.0),
        UFO_RADIUS,
        2.0,
        Color::GREEN,
    )
    .expect("Could not create mesh");

    Entity {
        e_type: EntityType::Ufo,
        pos: Vec2::new(x, y),
        vel: Vec2::new(vx, vy),
        angle,
        ang_vel: 0.0,
        mass: 100.0,
        mesh,
    }
}
