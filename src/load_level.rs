use crate::Entity;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

static mut CURRENT_LEVEL: u8 = 0;

#[derive(Debug, Deserialize)]
struct Missile {
    position: [i32; 2],
    angle: i32,
}

#[derive(Debug, Deserialize)]
struct UFO {
    position: [i32; 2],
    speed: [i32; 2],
}

#[derive(Debug, Deserialize)]
struct Planet {
    position: [i32; 2],
    radius: i32,
}

#[derive(Debug, Deserialize)]
struct Asteroid {
    position: [i32; 2],
    speed: [i32; 2],
    radius: i32,
}

#[derive(Debug, Deserialize)]
struct SpaceData {
    missile: Missile,
    ufo: UFO,
    planets: Vec<Planet>,
    asteroids: Vec<Asteroid>,
}

pub fn load_level_entities(file: &str) -> Vec<Entity> {
    let mut entities = Vec::new();
    let mut contents = String::new();

    // Update the current level
    unsafe {
        CURRENT_LEVEL += 1;
    }

    // Update the contents with file data
    let _ = File::open("levels/01.yaml")
        .expect("Unable to open level")
        .read_to_string(&mut contents);

    let space_data: SpaceData = serde_yaml::from_str(&contents).unwrap();

    println!("{:#?}", space_data);
    return entities;
}
