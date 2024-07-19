use crate::entity::*;
use crate::Context;
use orbital_assault::INITIAL_SPEED;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

static mut CURRENT_LEVEL: u8 = 1;

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
    speed: [f32; 2],
    radius: f32,
}

#[derive(Debug, Deserialize)]
struct SpaceData {
    missile: Missile,
    ufo: UFO,
    planets: Vec<Planet>,
    asteroids: Vec<Asteroid>,
}

pub fn load_level_entities() -> Vec<Entity> {
    let mut entities = Vec::new();
    let mut contents = String::new();

    // Update the current level
    let level_file = format!("levels/{:02}.yaml", unsafe { CURRENT_LEVEL });

    // Update the contents with file data
    let _ = File::open(&level_file)
        .expect(&format!("Unable to open level: {}", level_file))
        .read_to_string(&mut contents);
    let level_data: SpaceData = serde_yaml::from_str(&contents).unwrap();

    // Missile
    entities.push(create_missile(
        level_data.missile.position[0],
        level_data.missile.position[1],
        INITIAL_SPEED,
        level_data.missile.angle,
    ));

    // UFO
    entities.push(create_ufo(
        level_data.ufo.position[0],
        level_data.ufo.position[1],
        level_data.ufo.speed[0],
        level_data.ufo.speed[1],
    ));

    // Asteroids
    level_data.asteroids.iter().for_each(|a| {
        entities.push(create_asteroid(
            a.position[0],
            a.position[1],
            a.speed[0],
            a.speed[1],
            a.radius,
        ))
    });

    // Planets
    level_data
        .planets
        .iter()
        .for_each(|p| entities.push(create_planet(p.position[0], p.position[1], p.radius)));

    entities
}
