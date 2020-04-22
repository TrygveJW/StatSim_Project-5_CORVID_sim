
enum InfectedState{
    susceptible,
    infectious,
    recovered,
    dead,
}

struct Position {
    x: u32,
    y: u32,
}

struct Person{
    age: u32,
    state: InfectedState,
    Position: Position,
}