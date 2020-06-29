
pub fn calculate_displacement(u: f32, a: f32, t: f32) -> f32 {
    // TODO: move to utils file
    // TODO: Use generics
    // Implementation of suvat equation, s = displacement, u = intial velocity, a = acceleration, t = time
    u * t + 0.5 * a * t * t
}

pub fn calculate_new_velocity(u: f32, a: f32, t: f32) -> f32 {
    u + a * t
}
