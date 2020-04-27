mod sim_grid;
mod helpers;
mod person;
mod data_logging;
mod simulator;

use crate::simulator::Simulator;
use crate::helpers::*;
use crate::person::*;
use crate::data_logging::SimLogger;

use std::collections::HashMap;
use sim_grid::SimGrid;
use rand::Rng;




fn main() {

    
    let mut mort_rates = Vec::<(i32, f64)>::new();

    mort_rates.push((0, 0.0));
    mort_rates.push((20, 0.03));
    mort_rates.push((50, 0.5));
    mort_rates.push((60, 0.1));
    mort_rates.push((80, 0.2));

    // prøv med og sette capasitet på stuff i sim grid etterpå
    let mut sim = Simulator::new(
        250, // x size 
        250, // y size
        50000,  // num persons
        50, // num initially infected

        0.5, // infected chance
        14, // infected time
    
        mort_rates,

        true,
        true,
        0.0,
        true,
    );

    
    sim.populate();
    

    

    for n in 0..=500 {
        if n % 100 == 0{
            
            sim.print_stats();
            //println!("{}", sim.sim_grid.logger.map_logger.len())
        }
        if sim.step() {
            sim.print_stats();
            break;
        }
    }


    // let path = Path::new("out/Game_data_array");
    // let display = path.display();

    // let mut file = match File::create(&path) {
    //     Err(why) => panic!("couldn't create {}: {}", display, why.description()),
    //     Ok(file) => file,
    // };

    // // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    // match file.write_all(format!("{:?}", sim.format_sim_data_to_int_array()).as_bytes()) {
    //     Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
    //     Ok(_) => println!("successfully wrote to {}", display),
    // }
    
}
