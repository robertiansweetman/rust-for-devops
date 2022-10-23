# enums

enums have to be defined and then called as per

```rust

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// call Direction::Down and then combine them

enum Action {
    Move {
        direction: Direction,
        speed: u32,
    },
    Wait
}

fn main() {
    let action = Action::Move {
        direction: Direction::Down,
        speed: 10
    };
    match action {
        Action::Move { direction, speed } => {
            println!("Moving {:?} at speed {}", direction, speed);
        },
        Action::Wait => {
            println!("Waiting");
        }
    };
}

```

the {:?} is a placeholder for the enum value `direction` because the compiler needs to allow it to be printed in the {:?} format string in pringlin!().