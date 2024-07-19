use crate::entity::Entity;

pub fn distance(e1: &Entity, e2: &Entity) -> (f32) {
    let (p1, _) = e1.get_pose();
    let (p2, _) = e2.get_pose();

    (p2 - p1).length()
}
