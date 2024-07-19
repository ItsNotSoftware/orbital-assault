use orbital_assault::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    radius: f32,
    vel: Vec2,  // Bodyframe velocity
    theta: f32, // Angle in respect to the x-axis
    mass: f32,
}
impl Entity {
    pub fn get_entity_type(&self) -> EntityType {
        self.e_type
    }

    pub fn get_pose(&self) -> (Vec2, f32) {
        (self.pos, self.theta)
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
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

    pub fn apply_gravity(&mut self, entities: &Vec<Entity>, dt: f32) {
        for e in entities {
            let (p, _) = e.get_pose();

            // Distance and angle between entities
            let d = p - self.pos; // vector between the 2 entities
            let r_squared = d.length_squared(); // r^2
            let th = d.y.atan2(d.x); //angle

            // Gravity force
            let f = G * self.mass * e.get_mass() / r_squared;
            let force = f * Vec2::from_angle(th);

            self.apply_force(force, dt);
        }
    }

    pub fn is_out_of_bounds(&self) -> bool {
        self.pos.x > WINDOW_WIDTH
            || self.pos.x < 0.0
            || self.pos.y > WINDOW_HEIGHT
            || self.pos.y < 0.0
    }

    pub fn is_coliding(&self, entity: &Entity) -> bool {
        const HW: f32 = MISSILE_WIDTH / 2.0;
        const HH: f32 = MISSILE_HEIGHT / 2.0;

        // Verify correct entity types
        assert_eq!(self.e_type, EntityType::Missile);

        let (missile_pos, missile_angle) = self.get_pose(); // Top-left corner of the rectangle
        let (entity_pos, _) = entity.get_pose(); // Center of the circle

        // 1. Calculate the center of the missile
        let dir = Vec2::new(missile_angle.cos(), missile_angle.sin());
        let dx = dir * HW;
        let dy = Vec2::new(-dir.y, dir.x) * HH; // Perpendicular direction for height
        let missile_center = missile_pos + dx + dy;

        // 2. Transform the entity position to the missile's local coordinate system (translation + rotation)
        let relative_entity_pos = (entity_pos - missile_center).rotate(dir);

        // 3. Clamp the entity's center to the rectangle's bounds
        let clamped_x = relative_entity_pos.x.clamp(-HW, HW);
        let clamped_y = relative_entity_pos.y.clamp(-HH, HH);
        let nearest_point = Vec2::new(clamped_x, clamped_y);

        // 4. Calculate the distance between the circle's center and the nearest point
        let distance = (nearest_point - relative_entity_pos).length();

        // 5. Check if the distance is less than the radius of the circle
        distance < entity.get_radius()
    }
}

pub fn create_missile(x: f32, y: f32, v: f32, angle: f32) -> Entity {
    let angle = angle.to_radians();

    Entity {
        e_type: EntityType::Missile,
        pos: Vec2::new(x, y),
        vel: Vec2::from_angle(angle) * v,
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
