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

    infected_chance: f64,
    infected_time: i32,
    
}

impl<'a> Simulator {
    fn new(x_size: i32, y_size: i32, start_pop_size: i32, start_infected_size: i32,
        infected_chance: f64, infected_time: i32, mortality_rate: Vec<(i32,f64)>) -> Simulator{
        
        let sg = SimGrid::new(x_size, y_size, infected_chance, infected_time, mortality_rate);
        Simulator{
            persons: Vec::new(),
            sim_grid: sg,
            simulator_ticks: 0,

            x_size,
            y_size,

            start_pop_size,
            start_infected_size,

            infected_chance,
            infected_time,

        }

    }

    fn populate(&'a mut self){
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
        self.sim_grid.tick += 1;
    }



}

fn main() {

    println!("Hello, world!");
    let mut mort_rates = Vec::<(i32, f64)>::new();

    mort_rates.push((0, 0.0));
    mort_rates.push((20, 0.03));
    mort_rates.push((50, 0.5));
    mort_rates.push((60, 0.1));
    mort_rates.push((80, 0.2));


    let mut sim = Simulator::new(
        250, // x size 
        250, // y size
        10000,  // num persons
        1, // num initially infected

        0.5, // infected chance
        50, // infected time
    
        mort_rates,
    );

    
    sim.populate();
    

    

    for n in 0..500 {
        if n % 100 == 0{
            println!("\n\nCurr step {}", n);
            for inf_state in sim.sim_grid.stats.keys(){
                println!("aaaaaa");
                match inf_state{
                    InfectedState::Dead => println!("Num Dead \t{:#?}", sim.sim_grid.stats.get(inf_state)),
                    InfectedState::Recovered => println!("Num rec \t{:#?}", sim.sim_grid.stats.get(inf_state)),
                    InfectedState::Susceptible => println!("Num sus \t{:#?}", sim.sim_grid.stats.get(inf_state)),
                    _ => println!("Num infs \t{:#?}", sim.sim_grid.stats.get(inf_state)),
                }
            }
        }
        sim.step();
    }


    for inf_state in sim.sim_grid.stats.keys(){
        println!("aaaaaa");
        match inf_state{
            InfectedState::Dead => println!("Num Dead \t{:#?}", sim.sim_grid.stats.get(inf_state)),
            InfectedState::Recovered => println!("Num rec \t{:#?}", sim.sim_grid.stats.get(inf_state)),
            InfectedState::Susceptible => println!("Num sus \t{:#?}", sim.sim_grid.stats.get(inf_state)),
            _ => println!("Num infs \t{:#?}", sim.sim_grid.stats.get(inf_state)),
        }
    }
    
}
