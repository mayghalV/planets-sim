
use super::physics::{Planet, System, TimePosition};

use std::error::Error;
use std::fs;
use std::io::prelude::*;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    time_step: f32,
    total_time: f32,
    planets: Vec<Planet>,
}

pub fn read_config(filename: &str) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let config: Config = serde_json::from_str(&contents).unwrap();
    Ok(config)
}

pub fn read_config_and_simulate_system(config_path: &str) -> Vec<TimePosition> {
    let config = read_config(&config_path).unwrap();
    
    let mut system = System::new(config.planets);
    system.simulate_movement(config.total_time, config.time_step)
}

#[allow(dead_code)]
pub fn write_positions_to_json(path: &str, positions: &Vec<TimePosition>) {
    let s = serde_json::to_string(&positions).unwrap();

    let mut file = match fs::File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path, why),
        Ok(file) => file,
    };

    match file.write_all(s.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path, why),
        Ok(_) => println!("successfully wrote to {}", path),
    }
}