#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub enum TileState {
    Susceptible,
    Infectious(i32),
    Recovered,
    Dead,

    Free,
    Blocked,
    Err,
}
#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
