use crate::entity::*;
use orbital_assault::*;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct Missile {
    position: [f32; 2],
    angle: f32,
}

#[derive(Debug, Deserialize)]
struct UFO {
    position: [f32; 2],
    speed: [f32; 2],
}

#[derive(Debug, Deserialize)]
struct Planet {
    position: [f32; 2],
    radius: f32,
}

#[derive(Debug, Deserialize)]
struct Asteroid {
    position: [f32; 2],
    velocity: [f32; 2],
    radius: f32,
}

#[derive(Debug, Deserialize)]
struct SpaceData {
    missile: Missile,
    ufo: UFO,
    planets: Vec<Planet>,
    asteroids: Vec<Asteroid>,
}

pub fn load_level_entities(ctx: &mut Context, level: u8) -> Vec<Entity> {
    let mut entities = Vec::new();
    let mut contents = String::new();

    // Update the current level
    let level_file = format!("levels/{:02}.yaml", level);

    // Update the contents with file data
    let _ = File::open(&level_file)
        .expect(&format!("Unable to open level: {}", level_file))
        .read_to_string(&mut contents);
    let level_data: SpaceData = serde_yaml::from_str(&contents).unwrap();

    // UFO
    entities.push(create_ufo(
        ctx,
        level_data.ufo.position[0],
        level_data.ufo.position[1],
        level_data.ufo.speed[0],
        level_data.ufo.speed[1],
    ));

    // Asteroids
    level_data.asteroids.iter().for_each(|a| {
        entities.push(create_asteroid(
            ctx,
            a.position[0],
            a.position[1],
            a.velocity[0],
            a.velocity[1],
            a.radius,
        ))
    });

    // Planets
    level_data
        .planets
        .iter()
        .for_each(|p| entities.push(create_planet(ctx, p.position[0], p.position[1], p.radius)));

    // Missile (allways the last element)
    entities.push(create_missile(
        ctx,
        level_data.missile.position[0],
        level_data.missile.position[1],
        INITIAL_SPEED,
        level_data.missile.angle,
    ));

    entities
}
