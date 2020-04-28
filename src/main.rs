mod sim_grid;
mod helpers;
mod person;
mod data_logging;
mod simulator;

use crate::simulator::Simulator;
use crate::helpers::*;
use crate::person::*;
use crate::data_logging::SimLogger;



fn main() {

    
    

    // prøv med og sette capasitet på stuff i sim grid etterpå
    let mut sim = Simulator::new(
        250, // x size 
        250, // y size
        50000,  // num persons
        50, // num initially infected

        0.5, // infected chance
        14, // infected time
    

        true,
        true,
        0.0,
        true,
    );

    

    sim.run(1000, 100);


    
}
