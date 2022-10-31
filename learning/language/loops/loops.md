# loops

There are three types of loops:

## loops 'while true'

```rust
fn main() {
    let mut counter = 0;
    loop {
        println!("again!");
        counter += 1;
        if counter == 10 {
            break;
        }
    }
    println!("The value of counter is: {}", counter);
}
```

## while loops

```rust
fn main() {
    let mut counter = 0;
    while counter < 10 {
        println!("again!");
        counter += 1;
    }
    println!("The value of counter is: {}", counter);
}
```

## for loops

for loops in rust are basically a version of an iterator over a range where 0..10 is exclusive of 10 and 0..=10 is inclusive of 10.

```rust
fn main() {
    for number in 1..4 {
    println!("The value of number is: {}", number);
    }
    for i in 1..=10 {
        println!("The value of i is: {}", i);
    }
}
