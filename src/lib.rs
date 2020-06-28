#![allow(dead_code)]

use std::ops::Add;

pub const G: f32 = 6.67e-11f32;

#[derive(Debug)]
#[derive(PartialEq)]
struct Position(f32, f32);

#[derive(Debug)]
#[derive(PartialEq)]
struct Velocity(f32, f32);

#[derive(Debug)]
#[derive(PartialEq)]
struct Acceleration(f32, f32);


#[derive(Debug)]
#[derive(PartialEq)]
struct Force(f32, f32);


fn calculate_displacement(u: f32, a: f32, t: f32) -> f32 {
    // TODO: move to utils file
    // TODO: Use generics
    // Implementation of suvat equation, s = displacement, u = intial velocity, a = acceleration, t = time
    u * t + 0.5 * a * t * t
}

fn calculate_new_velocity(u: f32, a: f32, t: f32) -> f32 {
    u + a * t
}

struct Planet {
    id: String,
    mass: f32,
    radius: f32,
    position: Position,
    velocity: Velocity,
}

impl Position{
    fn difference_in_distance_from_point(&self, point: &Position) -> (f32, f32) {
        // Returns distance to travel from self to point
        let x_distance = point.0 - self.0;
        let y_distance = point.1 - self.1;
        (x_distance, y_distance)
    }

    fn distance_squared_from_point(&self, point: &Position) -> f32 {
        let (x_distance, y_distance) = self.difference_in_distance_from_point(point);
        x_distance.powi(2) + y_distance.powi(2)
    }

    fn distance_from_point(&self, point: &Position) -> f32 {
        self.distance_squared_from_point(point).sqrt()
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


impl Force {
    fn to_acceleration(&self, mass: f32) -> Acceleration{
        Acceleration(&self.0 / mass, &self.1 / mass)
    }
}

impl Add for Force {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}


impl Planet{
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

    fn calculate_gravitational_force(&self, other_planet: &Planet) -> Force {
        let force = self.calculate_gravitational_force_abs(other_planet);
        let angle_to_planet = self.position.angle_to_point(&other_planet.position);
        Force(force * angle_to_planet.cos() , force * angle_to_planet.sin())
    }

    fn apply_force(&mut self, force: &Force, time: f32){
        // TODO: This is quite generic so can use a trait here
        let accel = force.to_acceleration(self.mass);

        // Move
        let s_x = calculate_displacement(self.velocity.0, accel.0, time);
        let s_y = calculate_displacement(self.velocity.1, accel.1, time);
        self.position = Position(self.position.0 + s_x, self.position.1 + s_y);

        // Calculate new velocity too
        let v_x = calculate_new_velocity(self.velocity.0, accel.0, time);
        let v_y = calculate_new_velocity(self.velocity.1, accel.1, time);
        self.velocity = Velocity(v_x, v_y);

    }

}

// TODO: Convert equals to check approx equals

#[cfg(test)]
mod test_class_point{
    use super::*;

    fn point_factory(i: i32) -> Position {
        match i {
            0 => Position(0.0, 0.0),
            1 => Position(3.0, 4.0),
            2 => Position(-3.0, 4.0),
            3 => Position(3.0, -4.0),
            4 => Position(-3.0, -4.0),
            5 => Position(0.0, 6.0),
            6 => Position(0.0, -7.0),
            7 => Position(3.0, 0.0),
            8 => Position(-4.0, 0.0),
            9 => Position(-2.0, 8.0),
            10 => Position(4.0, 5.0),
            _ => panic!("Unknown point id"),
        }
    }


    #[test]
    fn test_distance_from_point(){
        let point_1 = point_factory(1);
        let point_2 = point_factory(0);
        
        assert_eq!(point_1.distance_from_point(&point_1), 0.0);

        assert_eq!(point_1.distance_from_point(&point_2), 5.0);
        assert_eq!(point_2.distance_from_point(&point_1), 5.0);
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
        
        assert_eq!(force_1 + force_2, Force(-4.2, 11.6))
    }
}





#[cfg(test)]
mod test_class_planet {
    use super::*;

    fn planet_factory(i: i32) -> Planet {
        match i {
            0 => Planet::new(String::from("planet_0"), 6.0, 5.0, Position(0.0, 0.0), Velocity(23.0, 43.0)),
            1 => Planet::new(String::from("planet_1"), 8.0e10, 10.0, Position(3.0, -4.0), Velocity(-3.0, 8.0)),
            2 => Planet::new(String::from("planet_2"), 7.0e2, 5.0, Position(-2.0, 3.0), Velocity(-3.0, 8.0)),
            3 => Planet::new(String::from("planet_3"), 6.0e2, 5.0, Position(8.0, 4.0), Velocity(-3.0, 8.0)),
            4 => Planet::new(String::from("planet_1"), 8.0, 10.0, Position(6.5, -4.0), Velocity(-3.0, 8.0)),
            //2 => Position(-3.0, 4.0),
            //3 => Position(3.0, -4.0),
            
            _ => panic!("Unknown point id"),
        }
    }

    #[test]
    fn test_new(){
        let planet = planet_factory(0);
        assert_eq!(planet.id, "planet_0");
        assert_eq!(planet.mass, 6.0);
        assert_eq!(planet.radius, 5.0);
        assert_eq!(planet.position, Position(0.0,0.0));
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
        assert_eq!(planet_1.position, Position(3.0, -0.5));

    }

}