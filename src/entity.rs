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
    theta: f32, // Angle in respect to the x-axis
    mass: f32,
    radius: f32,
}
impl Entity {
    pub fn get_pose(&self) -> (Vec2, f32) {
        (self.pos, self.theta)
    }

    pub fn get_mass(&self) -> f32 {
        self.mass
    }

    pub fn get_mesh(&self, ctx: &mut Context) -> Mesh {
        match self.e_type {
            EntityType::Missile => graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, MISSILE_WIDTH, MISSILE_HEIGHT),
                Color::RED,
            )
            .expect("Could not create mesh"),

            EntityType::Planet => graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(0.0, 0.0),
                self.radius,
                2.0,
                Color::MAGENTA,
            )
            .expect("Could not create mesh"),

            EntityType::Asteroid => graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(0.0, 0.0),
                self.radius,
                2.0,
                Color::BLACK,
            )
            .expect("Could not create mesh"),

            EntityType::Ufo => graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(0.0, 0.0),
                UFO_RADIUS,
                2.0,
                Color::GREEN,
            )
            .expect("Could not create mesh"),
        }
    }

    pub fn update_pos(&mut self, dt: f32) {
        self.theta = self.vel.y.atan2(self.vel.x);

        self.pos.x += self.vel.x * dt;
        self.pos.y += self.vel.y * dt;
    }

    pub fn apply_force(&mut self, force: Vec2, dt: f32) {
        self.vel.x += force.x / self.mass * dt;
        self.vel.y += force.y / self.mass * dt;
    }

    pub fn apply_gravity(&mut self, entities: &Vec<Entity>, dt: f32, ignore: usize) {
        for (i, e) in entities.iter().enumerate() {
            if i == ignore {
                continue;
            }

            let (p, _) = e.get_pose();

            // Distance and angle between entities
            let d = p - self.pos; // vector between the 2 entities
            let r = d.length(); // distance
            let th = d.y.atan2(d.x); //angle

            // Gravity force
            let f = G * self.mass * e.get_mass() / r.powi(2);
            let force = f * Vec2::new(th.cos(), th.sin());

            self.apply_force(force, dt);
        }
    }
}

pub fn create_missile(x: f32, y: f32, v: f32, angle: f32) -> Entity {
    let vx = v * angle.cos();
    let vy = v * angle.sin();

    Entity {
        e_type: EntityType::Missile,
        pos: Vec2::new(x, y),
        vel: Vec2::new(vx, vy),
        theta: angle,
        mass: MISSILE_MASS,
        radius: 0.0,
    }
}

pub fn create_planet(x: f32, y: f32, radius: f32) -> Entity {
    Entity {
        e_type: EntityType::Planet,
        pos: Vec2::new(x, y),
        vel: Vec2::new(0.0, 0.0),
        theta: 0.0,
        mass: PLANET_DENSITY * radius * radius * std::f32::consts::PI,
        radius,
    }
}

pub fn create_asteroid(x: f32, y: f32, vx: f32, vy: f32, radius: f32) -> Entity {
    Entity {
        e_type: EntityType::Asteroid,
        pos: Vec2::new(x, y),
        vel: Vec2::new(vx, vy),
        theta: 0.0,
        mass: ASTEROID_DENSITY * radius * radius * std::f32::consts::PI,
        radius,
    }
}

pub fn create_ufo(x: f32, y: f32, vx: f32, vy: f32) -> Entity {
    let angle = (vy / vx).atan();

    Entity {
        e_type: EntityType::Ufo,
        pos: Vec2::new(x, y),
        vel: Vec2::new(vx, vy),
        theta: angle,
        mass: 100.0,
        radius: UFO_RADIUS,
    }
}
