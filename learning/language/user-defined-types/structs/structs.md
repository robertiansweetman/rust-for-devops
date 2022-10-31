# structs

## unions structs

hardly used, don't worry about these

## tuple structs

individual fields are not named but are accessed by their position

say you wanted to represent RGB colours 

```rust
struct Color(u8, u8, u8);

let blue = Color(0, 0, 255);

println!("blue is {} {} {}", blue.0, blue.1, blue.2);
```

use tuple structs when you need to model data with < 4 - 5 attributes.

## named(?) structs

```rust
struct Player {
    name: String,
    health: u32,
    score: u32
}

fn bump_player_score(mut player: Player, score: u32) -> Player {
    player.score += 10;
}

fn main() {
    let name = "Alice".to_string();
    let player = Player {
        name,
        health: 100,
        score: 0
    };
    bump_player_score(player, 10);
}
```
Here structs are created in the same way as tuple structs but have curly braces and names for each field.

