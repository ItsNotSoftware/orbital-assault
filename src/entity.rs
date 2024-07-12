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
    radius: f32,
    mass: f32,
    mesh: Mesh,
}
impl Entity {
    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.draw(&self.mesh.clone(), self.pos.clone());
    }
}

pub fn create_missile(ctx: &mut Context, x: f32, y: f32, angle: f32) -> Entity {
    let mesh = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        Vec2::new(0.0, 0.0),
        MISSILE_RADIUS,
        2.0,
        Color::RED,
    )
    .expect("Could not create mesh");

    Entity {
        e_type: EntityType::Missile,
        pos: Vec2::new(x, y),
        vel: Vec2::new(0.0, 0.0),
        angle,
        ang_vel: 0.0,
        radius: MISSILE_RADIUS,
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
        Color::RED,
    )
    .expect("Could not create mesh");

    Entity {
        e_type: EntityType::Planet,
        pos: Vec2::new(x, y),
        vel: Vec2::new(0.0, 0.0),
        angle: 0.0,
        ang_vel: 0.0,
        radius,
        mass: PLANET_DENSITY * radius * radius * std::f32::consts::PI,
        mesh,
    }
}

pub fn create_asteroid(
    ctx: &mut Context,
    x: f32,
    y: f32,
    vt: f32,
    vn: f32,
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
        vel: Vec2::new(vt, vn),
        angle: 0.0,
        ang_vel: 0.0,
        radius: radious,
        mass: ASTEROID_DENSITY * radious * radious * std::f32::consts::PI,
        mesh,
    }
}

pub fn create_ufo(ctx: &mut Context, x: f32, y: f32, vt: f32, vn: f32, angle: f32) -> Entity {
    let mesh = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        Vec2::new(0.0, 0.0),
        20.0,
        2.0,
        Color::GREEN,
    )
    .expect("Could not create mesh");

    Entity {
        e_type: EntityType::Ufo,
        pos: Vec2::new(x, y),
        vel: Vec2::new(vt, vn),
        angle,
        ang_vel: 0.0,
        radius: 20.0,
        mass: 100.0,
        mesh,
    }
}
