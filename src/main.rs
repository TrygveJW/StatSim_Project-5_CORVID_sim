use rand::Rng;
///
/// Lag førs en simulator som virke i en fil, kan i ettertid
/// splitt den opp i flere moduler


/// Persjon og funcs


#[derive(Clone, Copy)]
enum InfectedState{
    Susceptible,
    Infectious,
    Recovered,
    Dead,
}

impl InfectedState{
    fn to_sus(self) -> Self {
        match self {
            InfectedState::Susceptible => InfectedState::Recovered,
            v => v
        }
    }
}


pub struct Position {
    pub x: usize,
    pub y: usize,
}



struct Person {
    age: u32,
    state: InfectedState,
    pos: Position,
    infected_tick: i32,
}


impl Person {
    // pub 
    pub fn act(&mut self, rng_val: i32, sim_grid: &mut SimGrid){
        let has_moved = match rng_val{
            1 => sim_grid.try_move(&mut self.pos, Direction::Up),
            2 => sim_grid.try_move(&mut self.pos, Direction::Up),
            3 => sim_grid.try_move(&mut self.pos, Direction::Up),
            4 => sim_grid.try_move(&mut self.pos, Direction::Up),
            _ => false
        };
    }

    // priv

    
}
/// grid og funcs



struct SimGrid<'a> {
    x_size: usize,
    y_size: usize,
    grid: Vec<Option<&'a mut InfectedState>>,

}

enum Direction{
    Up,
    Right,
    Left,
    Down,
}
///
/// 0 marks a free spot 
/// 1 is Susceptible
/// 2 is Infectious
/// 3 is Recovered
/// 4 is dead
impl<'a> SimGrid<'a> {
    fn new(x_size: usize, y_size: usize) -> SimGrid<'a> {
        let mut a : Vec<Option<&'a mut InfectedState>> = Vec::new();
        for _ in 0..(x_size*y_size) {
            a.push(Option::<& mut InfectedState>::None)
        }
        SimGrid {
            x_size,
            y_size,
            grid: a,
        }
    }
    // pub
    pub fn try_move(&mut self, pos: &mut Position, dir: Direction) -> bool{
        let maybe_new_pos = match dir {
            Direction::Up    => Position{x:pos.x, y: pos.y + 1},
            Direction::Down  => Position{x:pos.x, y: pos.y - 1},
            Direction::Right => Position{x:pos.x + 1, y: pos.y},
            Direction::Left  => Position{x:pos.x - 1, y: pos.y},
        };
        
        if self.is_free(&maybe_new_pos) {
            let mover_val = self.get_value_at(&pos);
            // match mover_val {
            //     None => (),
            //     Some(mut InfectedState) => InfectedState
            // }
            if let Some(ref InfectedState) = mover_val {
                InfectedState.to_sus();
            }
            self.set_value_at(&maybe_new_pos, mover_val);
            self.set_value_at(&pos, None);
            pos.x = maybe_new_pos.x;
            pos.y = maybe_new_pos.y;
            return true
        } else {
            return false
        }
    }

    pub fn update_sir_state(&self){

    }

    // priv
    fn set_value_at(&mut self, pos: &Position, val: Option<&'a mut InfectedState>){
        let new_index = self.get_index(pos);
        self.grid[new_index] = val;
    }

    fn get_value_at(&self, pos: &Position) -> Option<&'a mut InfectedState> {
        self.grid[self.get_index(pos)]
    }
    fn is_valid(&self, pos: &Position) -> bool {
        if pos.x >= self.x_size || pos.y >= self.y_size {
            return false
        } else {
            return true
        }
    }

    fn get_index(&self, pos: &Position) -> usize {
        (pos.y * self.y_size) + pos.x
       
    }

    fn is_free(&self, pos: &Position) ->  bool{
        if !self.is_valid(pos) {return false}
        let val = self.get_value_at(pos);
        return match val {
            None => true,
            _    => false,
        }
    }
}

fn main() {

    println!("Hello, world!");
}
