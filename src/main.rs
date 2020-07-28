mod planet_sim;
use planet_sim::file_methods::{read_config_and_simulate_system, write_positions_to_json};


fn main() {
    let positions = read_config_and_simulate_system("config/config.json" );
    write_positions_to_json("export/export.json", &positions);
}
