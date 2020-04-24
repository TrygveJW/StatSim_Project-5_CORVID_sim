mod sim_grid;
mod helpers;
mod person;


use crate::helpers::*;
use crate::person::*;

use sim_grid::SimGrid;
use rand::Rng;

struct Simulator<'a> {
    persons: Vec<Person>,
    sim_grid: SimGrid<'a>,
    simulator_ticks: i32,

    x_size: usize,
    y_size: usize,

    start_pop_size: i32,
    start_infected_size: i32,

}

impl<'a> Simulator<'_> {
    fn new(x_size: usize, y_size: usize, start_pop_size: i32, start_infected_size: i32) -> Simulator<'a>{
        let sg = SimGrid::new(x_size, y_size);
        Simulator{
            persons: Vec::new(),
            sim_grid: sg,
            simulator_ticks: 0,

            x_size,
            y_size,

            start_pop_size,
            start_infected_size,
        }
    }

    fn populate(&'a mut self){
        let mut rng = rand::thread_rng();
        let mut num_infected = 0;
        

        for n in 0..self.start_pop_size {
            let infected_state = if num_infected < self.start_infected_size {
                num_infected += 1;
                InfectedState::Infectious
            } else {
                InfectedState::Susceptible
            };
            let pos = Position{x: rng.gen_range(0, self.x_size), y: rng.gen_range(0, self.y_size)};
            let p = Person::new(infected_state, pos);
            self.persons.push(p);
        }
        for pers in self.persons.iter(){
            pers.init_move(&mut self.sim_grid)
            
        }

    }

    pub fn step(& mut self){
        let mut rng = rand::thread_rng();
        for pers in self.persons.iter_mut(){
            pers.act(rng.gen_range(0, 4),&mut self.sim_grid) 
        }
    }



}

fn main() {

    println!("Hello, world!");
}
