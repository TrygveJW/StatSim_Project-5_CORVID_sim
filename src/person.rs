use crate::helpers::*;
use crate::sim_grid::SimGrid;
use crate::simulator::Simulator;

pub struct Person {
    pub age: i32,
    pub state: TileState,
    pub pos: Position,
    pub infected_tick: i32,
    pub isolated: bool,
    pub infected_in_isolation: bool,
}

impl Person {
    pub fn new(
        state: TileState,
        pos: Position,
        isolated: bool,
        infected_in_isolation: bool,
    ) -> Person {
        Person {
            age: Person::gen_age(),
            state,
            pos,
            infected_tick: 0,
            isolated,
            infected_in_isolation,
        }
    }
    // pub
    pub fn act(&mut self, rng_val: i32, simulator: &mut Simulator) {
        if self.isolated {
            if self.infected_in_isolation {
                simulator.update_sir_state(self);
            }
        } else {
            simulator.update_sir_state(self);
            let has_moved = match rng_val {
                1 => simulator.try_move(self, Direction::Up),
                2 => simulator.try_move(self, Direction::Down),
                3 => simulator.try_move(self, Direction::Left),
                4 => simulator.try_move(self, Direction::Right),
                _ => false,
            };
        }
    }

    pub fn init_move(&self, sim_grid: &mut SimGrid) {
        sim_grid.set_value_at(&self.pos, self.state);
    }

    fn gen_age() -> i32 {
        35
    }

    // priv
}
