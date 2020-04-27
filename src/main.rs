mod sim_grid;
mod helpers;
mod person;


use crate::helpers::*;
use crate::person::*;

use std::collections::HashMap;
use sim_grid::SimGrid;
use rand::Rng;

struct Simulator {
    persons: Vec<Person>,
    sim_grid: SimGrid,
    simulator_ticks: i32,

    x_size: i32,
    y_size: i32,

    start_pop_size: i32,
    start_infected_size: i32,
    
}

impl<'a> Simulator {
    fn new(x_size: i32, y_size: i32, start_pop_size: i32, start_infected_size: i32,
        infected_chance: f64, infected_time: i32, mortality_rate: Vec<(i32,f64)>) -> Simulator{
        
        let sg = SimGrid::new(x_size, y_size, infected_chance, infected_time, mortality_rate, start_pop_size);
        Simulator{
            persons: Vec::with_capacity(start_pop_size as usize),
            sim_grid: sg,
            simulator_ticks: 0,

            x_size,
            y_size,

            start_pop_size,
            start_infected_size,

        }

    }

    fn populate(& mut self){
        let mut rng = rand::thread_rng();
        let mut num_infected = 0;
        

        for n in 0..self.start_pop_size {
            let infected_state = if num_infected < self.start_infected_size {
                num_infected += 1;
                InfectedState::Infectious(0)
            } else {
                InfectedState::Susceptible
            };

            let count = self.sim_grid.stats.entry(infected_state).or_insert(0);
            *count += 1;
        
            let mut tries = 0;
            while tries < 50{
                let pos = Position{x: rng.gen_range(0, self.x_size), y: rng.gen_range(0, self.y_size)};
                if self.sim_grid.is_free(&pos) {
                    let p = Person::new(infected_state, pos);
                    self.persons.push(p);
                    
                    break;
                }
            }
            
        }
        let sg = &mut self.sim_grid;
        for pers in self.persons.iter(){
            pers.init_move(sg)
        }
        

    }


    pub fn step(& mut self){
        let mut rng = rand::thread_rng();
        //let sg = & ;
        for pers in &mut self.persons{
            pers.act(rng.gen_range(0, 4), &mut self.sim_grid) 
        }
        self.sim_grid.next_tick();
    }


    pub fn print_stats(&self){
        

        let mut sus = 0;
        let mut inf = 0;
        let mut rec = 0;
        let mut ded = 0;


        if let Some(state) = self.sim_grid.stats.get(&InfectedState::Susceptible) {
            sus = *state;
        }
        if let Some(state) = self.sim_grid.stats.get(&InfectedState::Infectious(0)) {
            inf = *state;
        }
        if let Some(state) = self.sim_grid.stats.get(&InfectedState::Recovered) {
            rec = *state;
        }
        if let Some(state) = self.sim_grid.stats.get(&InfectedState::Dead) {
            ded = *state;
        }
        
        let death_rate = if rec == 0 {0.0} else {(ded as f64/rec as f64)  * 100.0};
        

        println!("\n################################################");
        println!("Step: {}\n", self.sim_grid.tick);
        println!("last trans num   {:#?}", self.avg_last_reprod_nums(100));
        println!("Death rate:      {:.3}%\n", death_rate);

        println!("Num sus \t{:#?}", sus);
        println!("Num inf \t{:#?}", inf);
        println!("Num rec \t{:#?}", rec);
        println!("Num Dead \t{:#?}", ded);

        println!("################################################");

    }

    fn format_sim_data_to_int_array(&self) -> Vec<Vec<i32>>{
        let mut ret : Vec<Vec<i32>> = Vec::new();
        for ts in &self.sim_grid.map_logger {
            let mut layer: Vec<i32> = Vec::new();
            for &tile in ts {
                match tile{
                    None => layer.push(0),
                    Some(inf_state) => {
                        match inf_state {
                            InfectedState::Susceptible => layer.push(1),
                            InfectedState::Infectious(_) => layer.push(2),
                            InfectedState::Recovered => layer.push(3),
                            InfectedState::Dead => layer.push(4),
                        }
                    }
                }
            }
            ret.push(layer);
        }
        ret
    }

 

    fn avg_last_reprod_nums(&self, num_to_avg : usize) -> f64{
        let rp_vec = &self.sim_grid.reproduction_nums;
        //println!("Num Dead \t{:#?}", rp_vec);

        let target_len = rp_vec.len();
        if num_to_avg >= target_len {
            return -1.0
        } else {
            let from = (target_len as i32 - num_to_avg as i32) as usize;
            rp_vec[from..].iter().sum::<i32>() as f64/(num_to_avg as f64)
        }
    
    }
    



}



use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
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
        1, // num initially infected

        0.5, // infected chance
        14, // infected time
    
        mort_rates,
    );

    
    sim.populate();
    

    

    for n in 0..=300 {
        if n % 100 == 0{
            
            sim.print_stats();
            println!("{}", sim.sim_grid.map_logger.len())
        }
        sim.step();
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
