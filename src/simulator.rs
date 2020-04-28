
use crate::helpers::*;
use crate::person::*;
use crate::data_logging::SimLogger;
use crate::sim_grid::SimGrid;

use rand::Rng;

struct simulation_data {
    
}

pub struct Simulator {
    sim_grid: SimGrid,
    simulator_ticks: i32,

    x_size: i32,
    y_size: i32,

    start_pop_size: i32,
    start_infected_size: i32,

    infected_chance: f64,
    infected_time: i32,
    mortality_rate: Vec<(i32,f64)>,

    quarantine: bool,
    can_die: bool,
    isolation: f64,
    infected_in_isolation: bool,

    done: bool,
    sim_tick: i32,
    pub logger: SimLogger,
    rng: rand::rngs::ThreadRng,
    
}

pub fn run_simulation(x_size: i32, y_size: i32, start_pop_size: i32, start_infected_size: i32,
    infected_chance: f64, infected_time: i32, mortality_rate: Vec<(i32,f64)>, quarantine: bool, can_die: bool, isolation: f64, infected_in_isolation: bool,){
        let sim = Simulator::new(x_size, y_size, start_pop_size, start_infected_size,
            infected_chance, infected_time, quarantine, can_die, isolation, infected_in_isolation);
        
    }

impl Simulator {
    pub fn new(x_size: i32, y_size: i32, start_pop_size: i32, start_infected_size: i32,
        infected_chance: f64, infected_time: i32, quarantine: bool, can_die: bool, isolation: f64, infected_in_isolation: bool,) -> Simulator{
        let logger = SimLogger::new(start_pop_size);
        let sg = SimGrid::new(x_size, y_size);

        let mut mortality_rate = Vec::<(i32, f64)>::new();

        mortality_rate.push((0, 0.0));
        mortality_rate.push((20, 0.03));
        mortality_rate.push((50, 0.5));
        mortality_rate.push((60, 0.1));
        mortality_rate.push((80, 0.2));

        Simulator{
            sim_grid: sg,
            simulator_ticks: 0,

            x_size,
            y_size,

            start_pop_size,
            start_infected_size,

            infected_chance,
            infected_time, 
            mortality_rate, 
            logger,

            quarantine,
            can_die,
            isolation,
            infected_in_isolation,

            done: false,
            rng: rand::thread_rng(),
            sim_tick: 0,            
            

        }

    }

    fn populate(& mut self) -> Vec<Person> {
        let mut ret  = Vec::with_capacity(self.start_pop_size as usize);
        let mut rng = rand::thread_rng();
        let mut num_infected = 0;
        let mut num_in_isolation = 0;
        let mut num_to_isolate: i32 = 0;

        if self.isolation > 0.0 && self.isolation < 1.0{
            num_to_isolate = (self.start_pop_size as f64 * self.isolation) as i32;
        }
        

        for n in 0..self.start_pop_size {
            let mut tile_state = TileState::Susceptible;
            let mut in_quarantine = false;
            if num_in_isolation < num_to_isolate{
                in_quarantine = true;
                num_in_isolation += 1;

            } else if num_infected < self.start_infected_size {
                num_infected += 1;
                tile_state = TileState::Infectious(0);
            } 

            self.logger.log_state_entry(tile_state);
        
            let tries = 0;
            while tries < 50{
                let pos = Position{x: rng.gen_range(0, self.x_size), y: rng.gen_range(0, self.y_size)};
                if self.sim_grid.is_free(&pos) {
                    let p = Person::new(tile_state, pos, in_quarantine, self.infected_in_isolation);
                    ret.push(p);
                    
                    break;
                }
            }
            
        }
        let sg = &mut self.sim_grid;
        for pers in ret.iter(){
            pers.init_move(sg)
        }
        ret
    }




    pub fn run(&mut self, max_steps : i64, log_every : i32){
        let mut persons = self.populate();
        for n in 0..max_steps{
            for pers in &mut persons {
                pers.act(self.rng.gen_range(0, 4), self);
            }
            
            self.sim_tick += 1;
    
            self.logger.log_curent_grid(&self.sim_grid);

            if let Some(state) = self.logger.num_in_states.get(&TileState::Infectious(0)) {
                let inf = *state;
                if inf == 0 {
                    self.logger.print_stats();
                    break;
                };
            }

            if self.sim_tick % log_every == 0{
            
                self.logger.print_stats();
            }
        }
    }

    pub fn try_move(&mut self, person: &mut Person, dir: Direction) -> bool{
        
        // Find the movement dir
        let pos = &person.pos;
        let maybe_new_pos = match dir {
            Direction::Up    => Position{x:pos.x, y: pos.y + 1},
            Direction::Down  => Position{x:pos.x, y: pos.y - 1},
            Direction::Right => Position{x:pos.x + 1, y: pos.y},
            Direction::Left  => Position{x:pos.x - 1, y: pos.y},
        };

        // complete the move if necessary
        if self.sim_grid.is_free(&maybe_new_pos) {
            
            self.sim_grid.set_value_at(&pos, TileState::Free);
            person.pos.x = maybe_new_pos.x;
            person.pos.y = maybe_new_pos.y;
            self.sim_grid.set_value_at(&maybe_new_pos, person.state);
            
            
            return true
        } else {
            return false
        }
    }

    pub fn update_sir_state(&mut self, person: &mut Person){
        match person.state {
            TileState::Infectious(_) => (
                if self.sim_tick - person.infected_tick > self.infected_time{

                    if let TileState::Infectious(n) = self.sim_grid.get_value_at(&person.pos) {
                        self.logger.log_reprod_num(&person, n);
                    }

                    if self.will_die(person){
                        self.update_state(person, TileState::Dead);
                    } else {
                        self.update_state(person, TileState::Recovered);
                    }
                }
            ),
            TileState::Susceptible => {
                if self.sim_grid.will_get_infected(&person){
                    if self.rng.gen::<f64>() < self.infected_chance{
                        self.update_state(person, TileState::Infectious(0));
                        person.infected_tick = self.sim_tick;
                    }
                    
                }
            }
            _ => (),
            
        }
    }

    fn update_state(&mut self, person: &mut Person, new_state: TileState){
        self.sim_grid.set_value_at(&person.pos, new_state);
        self.logger.log_state_change(&person, &new_state);
        person.state = new_state;
    }

    fn will_die(&mut self, person: &Person) -> bool{
        let mut die_prob = self.mortality_rate[self.mortality_rate.len()-1].1;
        for (age, prob) in &self.mortality_rate  {
            if *age < person.age {
                die_prob = *prob;
            }
        }
        let rn = self.rng.gen::<f64>();
        //println!("{}   {} ",rn,  die_prob );
        if rn < die_prob{
            return true
        } else {
            return false
        }
    }
}