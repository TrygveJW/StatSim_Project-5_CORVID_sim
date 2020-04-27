use crate::helpers::*;
use crate::person::*;
use crate::sim_grid::SimGrid;

use std::collections::HashMap;

pub struct SimLogger {
    pub num_in_states: HashMap<TileState, i32>,
    pub map_logger: Vec<Vec<TileState>>,
    pub reproduction_nums: Vec<i32>,
}

impl SimLogger {
    pub fn new(start_pop_size: i32) -> SimLogger {
        let mut num_in_states = HashMap::new();

        num_in_states.insert(TileState::Dead, 0);
        num_in_states.insert(TileState::Infectious(0), 0);
        num_in_states.insert(TileState::Recovered, 0);
        num_in_states.insert(TileState::Susceptible, 0);

        SimLogger {
            num_in_states,
            map_logger: Vec::new(),
            reproduction_nums: Vec::with_capacity(start_pop_size as usize),
        }
    }

    pub fn log_reprod_num(&mut self, person: &Person, reprod_num: i32) {
        self.reproduction_nums.push(reprod_num);
    }

    pub fn log_state_change(&mut self, person: &Person, new_state: &TileState) {
        let count = self.num_in_states.entry(person.state).or_insert(0);
        *count -= 1;

        let count = self.num_in_states.entry(*new_state).or_insert(0);
        *count += 1;
    }

    pub fn log_state_entry(&mut self, new_state: TileState){
        let count = self.num_in_states.entry(new_state).or_insert(0);
        *count += 1;
    }

    pub fn log_curent_grid(&mut self, sim_grid: &SimGrid){
        self.map_logger.push(sim_grid.grid.clone());
    }


    pub fn print_stats(&self){
        

        let mut sus = 0;
        let mut inf = 0;
        let mut rec = 0;
        let mut ded = 0;


        if let Some(state) = self.num_in_states.get(&TileState::Susceptible) {
            sus = *state;
            
        }
        if let Some(state) = self.num_in_states.get(&TileState::Infectious(0)) {
            inf = *state;
        }
        if let Some(state) = self.num_in_states.get(&TileState::Recovered) {
            rec = *state;
        }
        if let Some(state) = self.num_in_states.get(&TileState::Dead) {
            ded = *state;
        }
        
        let death_rate = if rec == 0 {0.0} else {(ded as f64/rec as f64)  * 100.0};
        

        println!("\n################################################");
        //println!("Step: {}\n", self.sim_grid.tick);
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
        for ts in &self.map_logger {
            let mut layer: Vec<i32> = Vec::new();
            for &tile in ts {
                match tile {
                    TileState::Blocked => layer.push(-1),
                    TileState::Susceptible => layer.push(1),
                    TileState::Infectious(_) => layer.push(2),
                    TileState::Recovered => layer.push(3),
                    TileState::Dead => layer.push(4),
                    _ => (),
                }
            }
            ret.push(layer);
        }
        ret
    }

    fn avg_last_reprod_nums(&self, num_to_avg : usize) -> f64{
        let rp_vec = &self.reproduction_nums;
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
