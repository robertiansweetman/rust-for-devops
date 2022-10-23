# match expressions

oh, these look awesome

So you can have a function that returns a match expression and then compare it to do something else

```rust
// function always returns 200
fn req_status() -> u32 {
    200
}
```

```rust
// match expression
fn main() {
    let status = req_status();

    match status {
        200 => println!("OK"),
        404 => println!("Not Found"),
            other => println!("Request failed with code: {}", other );
            // you can also use _ instead of other if you want to ignore the value
    }
}
```
