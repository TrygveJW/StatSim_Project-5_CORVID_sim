use crate::data_logging::SimLogger;
use crate::helpers::*;
use crate::person::*;
use crate::sim_grid::SimGrid;

use rand::Rng;


pub struct Simulator {
    sim_grid: SimGrid,
    simulator_ticks: i32,

    x_size: i32,
    y_size: i32,

    start_pop_size: i32,
    start_infected_size: i32,

    infected_chance: f64,
    infected_time: i32,

    quarantine: bool,
    quarantine_red_time: i32,
    quarantine_wall_pos: i32,
    can_die: bool,
    isolation: f64,
    infected_in_isolation: bool,

    done: bool,
    sim_tick: i32,
    pub logger: SimLogger,
    rng: rand::rngs::ThreadRng,
}


// Age: 80-90, Probability: 8%
// Age: 60-79, Probability: 20%
// Age: 40-59, Probability: 25%
// Age: 20-39, Probability: 30%
// Age: 0-19, Probability: 17%


fn get_random_age() -> i32 {
    let random = rand::thread_rng().gen_range(0, 101);

    match random {
        0..=17 => rand::thread_rng().gen_range(0, 20),
        17..=47 => rand::thread_rng().gen_range(20, 29),
        47..=72 => rand::thread_rng().gen_range(40, 59),
        72..=92 => rand::thread_rng().gen_range(60, 79),
        _       => rand::thread_rng().gen_range(80, 90),
    }

}

/*


    Age: 80+, Mortality: 14.8%
    Age: 70+, Mortality: 8%
    Age: 60+, Mortality: 3.6%
    Age: 50+, Mortality: 1.3%
    Age: 40+, Mortality: 0.4%
    Age: 30+, Mortality: 0.2%
    Age: 20+, Mortality: 0.2%
    Age: 10+, Mortality: 0.2%
    Age: 0+, Mortality: 0%


*/ 

fn will_person_die(person: &Person) -> bool{
    let die_probabilities: [(i32, i32); 5] = [(10, 0), (40, 2), (50, 4), (60,13), (70, 80)];
    let mut die_prob = 148;
    for (age, prob) in &die_probabilities {
        if *age < person.age {
            die_prob = *prob;
        }
    }
    //println!("{}   {} ",rn,  die_prob );
    if rand::thread_rng().gen_range(0, 1001) < die_prob {
        return true;
    } else {
        return false;
    }
}


impl Simulator {
    pub fn new(
        x_size: i32,
        y_size: i32,
        start_pop_size: i32,
        start_infected_size: i32,
        infected_chance: f64,
        infected_time: i32,
        can_die: bool,
    ) -> Simulator {
        let logger = SimLogger::new(start_pop_size);
        let sg = SimGrid::new(x_size, y_size);

        Simulator {
            sim_grid: sg,
            simulator_ticks: 0,

            x_size,
            y_size,

            start_pop_size,
            start_infected_size,

            infected_chance,
            infected_time,
            logger,

            quarantine: false,
            quarantine_red_time: 0,
            quarantine_wall_pos: 0,
            can_die,
            isolation: 0.0,
            infected_in_isolation: false,

            done: false,
            rng: rand::thread_rng(),
            sim_tick: 0,
        }
    }

    pub fn add_quarantine(&mut self, quarantine_red_time: i32, quarantine_wall_pos: i32) {
        self.quarantine = true;
        self.quarantine_red_time = quarantine_red_time;
        self.quarantine_wall_pos = quarantine_wall_pos;
        self.sim_grid.make_quarantine(quarantine_wall_pos)
    }

    pub fn add_isolation(&mut self, fracton_in_isolation: f64, can_get_infected_in_isolation: bool) {
        self.isolation = fracton_in_isolation;
        self.infected_in_isolation = can_get_infected_in_isolation;
    }

    


}


impl Simulator{
    fn populate(&mut self) -> Vec<Person> {
        let mut ret = Vec::with_capacity(self.start_pop_size as usize);
        let mut rng = rand::thread_rng();
        let mut num_infected = 0;
        let mut num_in_isolation = 0;
        let mut num_to_isolate: i32 = 0;

        if self.isolation > 0.0 && self.isolation < 1.0 {
            num_to_isolate = (self.start_pop_size as f64 * self.isolation) as i32;
        }

        for n in 0..self.start_pop_size {
            let mut tile_state = TileState::Susceptible;
            let mut in_quarantine = false;
            if num_in_isolation < num_to_isolate {
                in_quarantine = true;
                num_in_isolation += 1;
            } else if num_infected < self.start_infected_size {
                num_infected += 1;
                tile_state = TileState::Infectious(0);
            }

            self.logger.log_state_entry(tile_state);

            let tries = 0;
            while tries < 50 {
                let mut pos = Position {
                    x: rng.gen_range(0, self.x_size),
                    y: rng.gen_range(0, self.y_size),
                };

                if self.quarantine && tile_state == TileState::Infectious(0){
                    pos = Position {
                        x: rng.gen_range(0, self.quarantine_wall_pos),
                        y: rng.gen_range(0, self.y_size),
                    };
                } else {
                    pos = Position {
                        x: rng.gen_range(0, self.x_size),
                        y: rng.gen_range(0, self.y_size),
                    };
                }
                    
                
                if self.sim_grid.is_free(&pos) {
                    let p = Person::new(get_random_age(), tile_state, pos, in_quarantine, self.infected_in_isolation);
                    ret.push(p);

                    break;
                }
            }
        }
        let sg = &mut self.sim_grid;
        for pers in ret.iter() {
            pers.init_move(sg)
        }
        ret
    }

    pub fn run(&mut self, max_steps: i64, log_every: i32) {
        let mut persons = self.populate();
        for n in 0..max_steps {
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
            if self.quarantine{
                self.sim_grid.decrease_quarantine_wall(1);
            }

            if self.sim_tick % log_every == 0 {
                self.logger.print_stats();
            }
        }
    }

    pub fn try_move(&mut self, person: &mut Person, dir: Direction) -> bool {
        // Find the movement dir
        let pos = &person.pos;
        let maybe_new_pos = match dir {
            Direction::Up => Position {
                x: pos.x,
                y: pos.y + 1,
            },
            Direction::Down => Position {
                x: pos.x,
                y: pos.y - 1,
            },
            Direction::Right => Position {
                x: pos.x + 1,
                y: pos.y,
            },
            Direction::Left => Position {
                x: pos.x - 1,
                y: pos.y,
            },
        };

        // complete the move if necessary
        if self.sim_grid.is_free(&maybe_new_pos) {
            self.sim_grid.set_value_at(&pos, TileState::Free);
            person.pos.x = maybe_new_pos.x;
            person.pos.y = maybe_new_pos.y;
            self.sim_grid.set_value_at(&maybe_new_pos, person.state);

            return true;
        } else {
            return false;
        }
    }

    pub fn update_sir_state(&mut self, person: &mut Person) {
        match person.state {
            TileState::Infectious(_) => {
                if self.sim_tick - person.infected_tick > self.infected_time {
                    if let TileState::Infectious(n) = self.sim_grid.get_value_at(&person.pos) {
                        self.logger.log_reprod_num(&person, n);
                    }

                    if will_person_die(person) {
                        self.update_state(person, TileState::Dead);
                    } else {
                        self.update_state(person, TileState::Recovered);
                    }
                }
            }
            TileState::Susceptible => {
                if self.sim_grid.will_get_infected(&person) {
                    if self.rng.gen::<f64>() < self.infected_chance {
                        self.update_state(person, TileState::Infectious(0));
                        person.infected_tick = self.sim_tick;
                    }
                }
            }
            _ => (),
        }
    }

    fn update_state(&mut self, person: &mut Person, new_state: TileState) {
        self.sim_grid.set_value_at(&person.pos, new_state);
        self.logger.log_state_change(&person, &new_state);
        person.state = new_state;
    }
}