use crate::data_logging::SimLogger;
use crate::helpers::*;
use crate::person::Person;

use backtrace::Backtrace;
use rand::Rng;
use std::collections::HashMap;

pub struct SimGrid {
    x_size: i32,
    y_size: i32,
    pub grid: Vec<TileState>,
    quarantine_wall_x_pos: i32,
}

impl SimGrid {
    pub fn new(x_size: i32, y_size: i32) -> SimGrid {
        SimGrid {
            x_size,
            y_size,
            grid: (0..=(x_size * y_size)).map(|_x| TileState::Free).collect(),
            quarantine_wall_x_pos: -1,
        }
    }

    pub fn make_quarantine(&mut self, wall_x_cord: i32){
        if self.quarantine_wall_x_pos > 0 || wall_x_cord < 0 || wall_x_cord < self.x_size{
            for n in 0..self.y_size{
                self.set_value_at(&Position{x:wall_x_cord, y: n}, TileState::Blocked)
            }
        }
        
    }

    pub fn decrease_quarantine_wall(&mut self, decrese_num: i32){
        let midpoint: i32 = self.y_size / 2;
        let top = Position{x:self.quarantine_wall_x_pos, y: 0};
        let bottom = Position{x:self.quarantine_wall_x_pos, y: self.y_size-1};

        if self.get_value_at(&top) == TileState::Blocked {
            
        }

        for n in 0..=self.y_size{
            if self.get_value_at(&bottom) != TileState::Blocked || n == midpoint {
                self.set_value_at(&Position{x:self.quarantine_wall_x_pos, y: n-1}, TileState::Free);

            }

            if self.get_value_at(&top) != TileState::Blocked {
                self.set_value_at(&Position{x:self.quarantine_wall_x_pos, y: n-1}, TileState::Blocked);
            }
        }

        
    }

    pub fn will_get_infected(&mut self, person: &Person) -> bool {
        let pos = &person.pos;
        if self.try_get_infected_by(&Position {
            x: pos.x,
            y: pos.y + 1,
        }) {
            return true;
        };
        if self.try_get_infected_by(&Position {
            x: pos.x,
            y: pos.y - 1,
        }) {
            return true;
        };
        if self.try_get_infected_by(&Position {
            x: pos.x + 1,
            y: pos.y,
        }) {
            return true;
        };
        if self.try_get_infected_by(&Position {
            x: pos.x - 1,
            y: pos.y,
        }) {
            return true;
        };
        false
    }

    fn try_get_infected_by(&mut self, pos: &Position) -> bool {
        if let TileState::Infectious(n) = self.get_value_at(pos) {
            self.set_value_at(pos, TileState::Infectious(n + 1));
            return true;
        }
        false
    }

    pub fn set_value_at(&mut self, pos: &Position, val: TileState) {
        let new_index = self.get_index(pos);
        self.grid[new_index as usize] = val;
    }

    pub fn get_value_at(&self, pos: &Position) -> TileState {
        if self.is_valid(pos) {
            return self.grid[self.get_index(pos) as usize];
        };
        //println!("ERROR WRONG INDEX");
        return TileState::Err;
    }

    fn is_valid(&self, pos: &Position) -> bool {
        if pos.x >= self.x_size || pos.y >= self.y_size || pos.x < 0 || pos.y < 0 {
            return false;
        } else {
            return true;
        }
    }

    fn get_index(&self, pos: &Position) -> i32 {
        (pos.y * self.x_size) + pos.x
    }

    pub fn is_free(&self, pos: &Position) -> bool {
        if let TileState::Free = self.get_value_at(&pos) {
            //println!("Valid INDEX");
            return true;
        } else {
            false
        }
    }
}
