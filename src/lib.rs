
mod planet_sim;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pymodule]
fn planet_sim(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_config_and_simulate_system))?;
    Ok(())
}

#[pyfunction]
pub fn read_config_and_simulate_system(config_path: &str) -> Vec<planet_sim::physics::TimePosition> {
    planet_sim::file_methods::read_config_and_simulate_system(config_path)
}