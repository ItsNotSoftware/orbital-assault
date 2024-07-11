use orbital_assault::*;

#[derive(Debug, Clone, Copy)]
pub enum EntityType {
    Missile,
    Planet,
    Asteroid,
    Ufo,
}

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    e_type: EntityType,
    pos: Vec2,
    vel: Vec2,  // Bodyframe velocity
    angle: f32, // Angle in respect to the x-axis
    ang_vel: f32,
    radius: f32,
    mass: f32,
}
impl Entity {
    pub fn get_pos(self) -> Vec2 {
        self.pos
    }

    pub fn get_mesh(self, ctx: &mut Context) -> Mesh {
        graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            100.0,
            2.0,
            Color::WHITE,
        )
        .expect("Could not create mesh")
    }
}

pub fn create_missile(x: f32, y: f32, angle: f32) -> Entity {
    Entity {
        e_type: EntityType::Missile,
        pos: Vec2::new(x, y),
        vel: Vec2::new(0.0, 0.0),
        angle,
        ang_vel: 0.0,
        radius: MISSILE_RADIUS,
        mass: MISSILE_MASS,
    }
}

pub fn create_planet(x: f32, y: f32, radius: f32) -> Entity {
    Entity {
        e_type: EntityType::Planet,
        pos: Vec2::new(x, y),
        vel: Vec2::new(0.0, 0.0),
        angle: 0.0,
        ang_vel: 0.0,
        radius,
        mass: PLANET_DENSITY * radius * radius * std::f32::consts::PI,
    }
}

pub fn create_asteroid(x: f32, y: f32, vt: f32, vn: f32, radious: f32) -> Entity {
    Entity {
        e_type: EntityType::Asteroid,
        pos: Vec2::new(x, y),
        vel: Vec2::new(vt, vn),
        angle: 0.0,
        ang_vel: 0.0,
        radius: radious,
        mass: ASTEROID_DENSITY * radious * radious * std::f32::consts::PI,
    }
}

pub fn create_ufo(x: f32, y: f32, vt: f32, vn: f32, angle: f32) -> Entity {
    Entity {
        e_type: EntityType::Ufo,
        pos: Vec2::new(x, y),
        vel: Vec2::new(vt, vn),
        angle,
        ang_vel: 0.0,
        radius: 20.0,
        mass: 100.0,
    }
}
