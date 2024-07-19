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
    v_theta: f32,
    mass: f32,
    mesh: Mesh,
}
impl Entity {
    pub fn get_pose(&self) -> (Vec2, f32) {
        (self.pos, self.theta)
    }

    pub fn get_mass(&self) -> f32 {
        self.mass
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.draw(&self.mesh, self.pos);
    }

    pub fn update_pos(&mut self, dt: f32) {
        self.pos.x += self.vel.x * dt;
        self.pos.y += self.vel.y * dt;
    }

    pub fn apply_force(&mut self, force: Vec2, dt: f32) {
        self.vel.x += force.x / self.mass * dt;
        self.vel.y += force.y / self.mass * dt;
    }

    pub fn apply_gravity(&mut self, entities: &Vec<Entity>, dt: f32, ignore: usize) {
        const G: f32 = 6.67430 * 10e-6; // In this universe gravity is 11 times stronger

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
        theta: angle,
        v_theta: 0.0,
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
        theta: 0.0,
        v_theta: 0.0,
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
        theta: 0.0,
        v_theta: 0.0,
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
        theta: angle,
        v_theta: 0.0,
        mass: 100.0,
        mesh,
    }
}
