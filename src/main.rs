mod lib;
use lib::{Planet};

use std::error::Error;
use std::fs;

fn main() {
    let planet = read_config("config.json").unwrap();
    println!("{:?}", planet)
}



pub fn read_config(filename: &str) -> Result<Planet, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let planet: Planet = serde_json::from_str(&contents).unwrap();
    Ok(planet)
}