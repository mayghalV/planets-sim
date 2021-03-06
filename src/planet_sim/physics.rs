use std::ops::{Add, Mul};
use std::clone::Clone;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use pyo3::prelude::*;

use super::utils::{calculate_displacement, calculate_new_velocity};


pub const G: f32 = 6.67e-11f32;


#[pyclass]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Position {
    // Moved away from tuple struct here to be able to send over pyo3
    #[pyo3(get)] pub x: f32,
    #[pyo3(get)] pub y: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Velocity(f32, f32);

#[derive(Debug, PartialEq)]
pub struct Acceleration(f32, f32);


#[derive(Debug, PartialEq)]
pub struct Force(pub f32, pub f32);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Planet {
    pub id: String,
    mass: f32,
    radius: f32,
    position: Position,
    velocity: Velocity,
}

pub struct System {
    planets: Vec<Planet>,
}


#[pyclass]
#[derive(Serialize, Debug)]
pub struct TimePosition {
    #[pyo3(get)] pub time: f32,
    #[pyo3(get)] pub positions: HashMap<String, Position>,
}


impl Position{
    fn new(x: f32, y: f32) -> Position {
        Position{x: x, y: y}
    }

    fn difference_in_distance_from_point(&self, point: &Position) -> (f32, f32) {
        // Returns distance to travel from self to point
        let x_distance = point.x - self.x;
        let y_distance = point.y - self.y;
        (x_distance, y_distance)
    }

    fn distance_squared_from_point(&self, point: &Position) -> f32 {
        let (x_distance, y_distance) = self.difference_in_distance_from_point(point);
        x_distance.powi(2) + y_distance.powi(2)
    }

    fn angle_to_point(&self, point: &Position) -> f32 {
        // Gives the angle from self to point, where a 0 angle is the x axis
        let (x_distance, y_distance) = self.difference_in_distance_from_point(point);
        if x_distance == 0.0 {
        
            if y_distance > 0.0 {
                return std::f32::consts::PI / 2.0;
            } else if y_distance < 0.0 {
                return -1.0 * std::f32::consts::PI / 2.0;
            } else {
                return 0.0;
            }
        }

        if y_distance == 0.0 {
            if x_distance < 0.0 {
                return -1.0 * std::f32::consts::PI;
            }
        }

        let tan_theta = y_distance / x_distance;
        let theta = tan_theta.atan();

        if x_distance < 0.0 {
            // Shifts distance by PI if in left quadrants
            return theta + y_distance.signum() * std::f32::consts::PI;
        }

        theta
    }

}

impl TimePosition {
    pub fn new(time: f32, positions: HashMap<String, Position>) -> TimePosition {
        TimePosition{time: time, positions: positions}
    }
}


impl Force {
    fn to_acceleration(&self, mass: f32) -> Acceleration{
        Acceleration(&self.0 / mass, &self.1 / mass)
    }

    pub fn append_force(&mut self, force: &Force) {
        self.0 = self.0 + force.0;
        self.1 = self.1 + force.1;
    }

}

impl Add for &Force {
    type Output = Force;

    fn add(self, other: &Force) -> Force {
        Force(self.0 + other.0, self.1 + other.1)
    }
}

impl Mul<f32> for &Force {
    // The multiplication of rational numbers is a closed operation.
    type Output = Force;

    // TODO: Should use generics here, but only when force is a generic too
    fn mul(self, rhs: f32) -> Force {
        Force(self.0 * rhs, self.1 * rhs)
    }
}


impl Planet{

    #[allow(dead_code)]
    pub fn new(id: String, mass: f32, radius: f32, position: Position, velocity: Velocity) -> Planet {
        Planet{id: id, mass: mass, radius: radius, position: position, velocity: velocity}
    }

    fn calculate_gravitational_force_abs(&self, other_planet: &Planet) -> f32 {
        let r_squared = self.position.distance_squared_from_point(&other_planet.position);
        if r_squared == 0.0 {
            return 0.0;
        }

        // Order of multiplication of the floats seems to matter
        if self.mass > other_planet.mass{
            return self.mass*other_planet.mass*G/r_squared
        }
        other_planet.mass*self.mass*G/r_squared
    }

    pub fn calculate_gravitational_force(&self, other_planet: &Planet) -> Force {
        // Force applied on self as a result of other_planet
        let force = self.calculate_gravitational_force_abs(other_planet);
        let angle_to_planet = self.position.angle_to_point(&other_planet.position);
        Force(force * angle_to_planet.cos(), force * angle_to_planet.sin())
    }

    pub fn apply_force(&mut self, force: &Force, time: f32){
        // TODO: This is quite generic so can use a trait here
        let accel = force.to_acceleration(self.mass);

        // Move
        let s_x = calculate_displacement(self.velocity.0, accel.0, time);
        let s_y = calculate_displacement(self.velocity.1, accel.1, time);
        self.position = Position::new(self.position.x + s_x, self.position.y + s_y);

        // Calculate new velocity too
        let v_x = calculate_new_velocity(self.velocity.0, accel.0, time);
        let v_y = calculate_new_velocity(self.velocity.1, accel.1, time);
        self.velocity = Velocity(v_x, v_y);

    }
    
    pub fn get_position_clone(&self) -> Position {
        self.position.clone()
    }

}


impl System {
    pub fn new(planets: Vec<Planet>) -> System {
        System{ planets: planets }
    }

    fn build_force_dict(&self) -> HashMap<String, Force> {
        // Calculates the net force on each planet
        let mut force_dict: HashMap<String, Force> = HashMap::new();
        
        
        for planet_1 in self.planets.iter(){
            for planet_2 in self.planets.iter(){
                if planet_1.id < planet_2.id {
                    let force_on_planet_1 = planet_1.calculate_gravitational_force(planet_2);
                    let force_on_planet_2 = &force_on_planet_1 * -1.0;
                    
                    let force_1 = force_dict.entry(planet_1.id.clone()).or_insert(Force(0.0, 0.0));
                    force_1.append_force(&force_on_planet_1);
                    
                    let force_2 = force_dict.entry(planet_2.id.clone()).or_insert(Force(0.0, 0.0));
                    force_2.append_force(&force_on_planet_2);
                }
            
            }
        }
    
        force_dict
    }

    pub fn simulate_movement(&mut self, total_time: f32, time_step: f32) -> Vec<TimePosition>{
        let mut time_elapsed: f32 = 0.0;
        let mut positions: Vec<TimePosition> = Vec::new();
        
        // Save initial positions
        positions.push(TimePosition::new(time_elapsed, self.create_position_map(&self.planets)));
        
        while time_elapsed < total_time {
            let mut force_dict = self.build_force_dict();
            for planet in self.planets.iter_mut(){
                let force = force_dict.remove(&planet.id).unwrap();
                planet.apply_force(&force, time_step);
            }
            time_elapsed += time_step;
            positions.push(TimePosition::new(time_elapsed, self.create_position_map(&self.planets)));
        }
        
        positions

    }

    fn create_position_map(&self, planet_vec: &Vec<Planet>) -> HashMap<String, Position> {
        let mut positions: HashMap<String, Position> = HashMap::new();
        for planet in planet_vec {
            positions.insert(planet.id.clone(), planet.get_position_clone() );
        } 
        positions
    }


}



// TODO: Convert equals to check approx equals

#[cfg(test)]
mod test_class_point{
    use super::*;

    fn point_factory(i: i32) -> Position {
        match i {
            0 => Position::new(0.0, 0.0),
            1 => Position::new(3.0, 4.0),
            2 => Position::new(-3.0, 4.0),
            3 => Position::new(3.0, -4.0),
            4 => Position::new(-3.0, -4.0),
            5 => Position::new(0.0, 6.0),
            6 => Position::new(0.0, -7.0),
            7 => Position::new(3.0, 0.0),
            8 => Position::new(-4.0, 0.0),
            9 => Position::new(-2.0, 8.0),
            10 => Position::new(4.0, 5.0),
            _ => panic!("Unknown point id"),
        }
    }


    #[test]
    fn test_angle_to_point(){
        let config = [
            (0, 0, 0.0),
            (0, 1, 0.9272952),
            (0, 5, std::f32::consts::PI/2.0),
            (0, 6, -1.0 * std::f32::consts::PI/2.0),
            (0, 7, 0.0),
            (0, 8, -1.0 * std::f32::consts::PI),
            (0, 2, 2.2142975),
            (0, 3, -0.9272952),
            (0, 4, -2.2142975),
            (9, 10, -0.4636476),
            (10, 9, 2.6779451),
            
        ];
        for tup in config.iter() {

            let point_1 = point_factory(tup.0);
            let point_2 = point_factory(tup.1);
            let expected_angle = tup.2;
            assert_eq!(point_1.angle_to_point(&point_2), expected_angle);

        }

    }

}

#[cfg(test)]
mod test_class_force {
    use super::*;

    #[test]
    fn test_to_acceleration(){
        let f = Force(4.0, 6.2);
        let a = f.to_acceleration(2.0);
        assert_eq!(a, Acceleration(2.0, 3.1));
    }

    #[test]
    fn test_add(){
        let force_1 = Force(1.0, 4.2);
        let force_2 = Force(-5.2, 7.4);
        
        assert_eq!(&force_1 + &force_2, Force(-4.2, 11.6))
    }

    #[test]
    fn test_mul(){
        let force_1 = &Force(1.0, 4.2) * -2.0;
        assert_eq!(force_1, Force(-2.0, -8.4));
    }

    #[test]
    fn test_append_force(){
        let mut force_1 = Force(1.0, 4.2);
        let force_2 = Force(-2.0, 2.0);

        force_1.append_force(&force_2);
        assert_eq!(force_1, Force(-1.0, 6.2));
    }
}


#[cfg(test)]
mod test_class_planet {
    use super::*;

    fn planet_factory(i: i32) -> Planet {
        match i {
            0 => Planet::new(String::from("planet_0"), 6.0, 5.0, Position::new(0.0, 0.0), Velocity(23.0, 43.0)),
            1 => Planet::new(String::from("planet_1"), 8.0e10, 10.0, Position::new(3.0, -4.0), Velocity(-3.0, 8.0)),
            2 => Planet::new(String::from("planet_2"), 7.0e2, 5.0, Position::new(-2.0, 3.0), Velocity(-3.0, 8.0)),
            3 => Planet::new(String::from("planet_3"), 6.0e2, 5.0, Position::new(8.0, 4.0), Velocity(-3.0, 8.0)),
            4 => Planet::new(String::from("planet_1"), 8.0, 10.0, Position::new(6.5, -4.0), Velocity(-3.0, 8.0)),    
            _ => panic!("Unknown point id"),
        }
    }

    #[test]
    fn test_new(){
        let planet = planet_factory(0);
        assert_eq!(planet.id, "planet_0");
        assert_eq!(planet.mass, 6.0);
        assert_eq!(planet.radius, 5.0);
        assert_eq!(planet.position, Position::new(0.0,0.0));
        assert_eq!(planet.velocity, Velocity(23.0,43.0));
    }


    #[test]
    fn test_calculate_gravitational_force_abs(){
        // TODO: This should really mock out the calculate distance but it's not working at the moment
        let parameters = [
            (1, 2, 50.475674),
            (2, 1, 50.475674),
            (1, 3, 35.973034),
            (3, 1, 35.973034),
            
        ];

        for param in parameters.iter() {
            let planet_a = planet_factory(param.0);
            let planet_b: Planet = planet_factory(param.1);
            
            assert_eq!(planet_a.calculate_gravitational_force_abs(&planet_b), param.2);
        }

    }

    #[test]
    fn test_calculate_gravitational_force(){

        let parameters = [
            (1, 2, Force(-29.338387, 41.073746)),
            (2, 1, Force(29.338387, -41.073746)),
            (2, 2, Force(0.0, 0.0)),
            (1, 3, Force(19.06567, 30.505072)),
            
        ];

        for param in parameters.iter() {
            let planet_a = planet_factory(param.0);
            let planet_b: Planet = planet_factory(param.1);
            
            assert_eq!(planet_a.calculate_gravitational_force(&planet_b), param.2);
        }
        
    }

    #[test]
    fn test_apply_force(){
        let mut planet_1 = planet_factory(4);
        planet_1.apply_force(&Force(10.0, -50.0), 2.0);

        assert_eq!(planet_1.velocity, Velocity(-0.5, -4.5));
        assert_eq!(planet_1.position, Position::new(3.0, -0.5));

    }

}


#[cfg(test)]
mod test_class_system{
    use super::*;

    fn planet_factory(i: i32) -> Planet {
        match i {
            0 => Planet::new(String::from("planet_0"), 6.0e5, 5.0, Position::new(0.0, 0.0), Velocity(23.0, 43.0)),
            1 => Planet::new(String::from("planet_2"), 7.0e5, 5.0, Position::new(-10.0, 3.0), Velocity(-3.0, 8.0)),
            2 => Planet::new(String::from("planet_3"), 8.0e5, 5.0, Position::new(8.0, 4.0), Velocity(-3.0, 8.0)),
            _ => panic!("Unknown point id"),
        }
    }

    fn get_test_system() -> System {
        let planets: Vec<Planet> = vec![0,1,2].into_iter().map(|x| planet_factory(x)).collect();
        System::new(planets)
    }


    #[test]
    fn test_build_force_dict(){
        // TODO: This should probably mock instead of actual numbers...
        let system = get_test_system();
        let force_dict = system.build_force_dict();
        assert_eq!(force_dict.len(), 3);
        assert_eq!(force_dict.get("planet_0").unwrap(), &Force(0.11177966, 0.25282595));
    }
    
    #[test]
    fn test_mocking(){
        
    }

}