mod physics;
use physics::{Planet, System};

use std::error::Error;
use std::fs;
use std::io::prelude::*;

use serde::{Serialize, Deserialize};

fn main() {
    let config = read_config("config.json").unwrap();
    println!("{:?}", config);
    
    let mut system = System::new(config.planets);
    let positions = system.simulate_movement(5.0, 1.0);
    println!("{:?}", positions);

    let s = serde_json::to_string(&positions).unwrap();
    let path = String::from("export.json");

    let mut file = match fs::File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path, why),
        Ok(file) => file,
    };

    match file.write_all(s.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path, why),
        Ok(_) => println!("successfully wrote to {}", path),
    }
    

}

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
