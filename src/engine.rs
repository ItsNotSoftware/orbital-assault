use entity::Entity;
use orbital_assault::*;

pub fn distance(e1: &Entity, e2: &Entity) -> f32 {
    let (p1, _) = e1.get_pose();
    let (p2, _) = e2.get_pose();

    (p1 - p2).length()
}

pub fn gravity_force(e1: &Entity, e2: &Entity) -> Vec2 {
    const G: f32 = 6.67430; // In this universe gravity is 11 times stronger

    let f = G * e1.get_mass() * e2.get_mass() / distamce(e1, e2).powi(2);
}
