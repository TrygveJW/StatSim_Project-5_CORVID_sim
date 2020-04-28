mod data_logging;
mod helpers;
mod person;
mod sim_grid;
mod simulator;

use crate::data_logging::SimLogger;
use crate::helpers::*;
use crate::person::*;
use crate::simulator::Simulator;

fn main() {
    // prøv med og sette capasitet på stuff i sim grid etterpå
    let mut sim = Simulator::new(
        250,   // x size
        250,   // y size
        50000, // num persons
        50,    // num initially infected
        0.5,   // infected chance
        14,    // infected time
        true, true, 0.0, true,
    );

    sim.run(1000, 100);
}
