# conditionals

basically the same as other languages with if {} else {}

if/else conditionals in rust always return a value, basically whatever remains in the last line inside the braces becomes the value. It could also be an empty() unit type.

NOTE: both if and else branches must return the same unit type.

we don't need paratheses around the if block

we can also assing the value of if/else blocks to a variable

```rust

fn main() {
    let result = if 1 ==2 {
        "Wait, what?"
    } else {
        "Phew, Rust makes sense"
    };

    println!("Oh good? {}", result)
}
```

The ; in the above code is important 'cause assigning values that have been returned from an if/else expression need to end with a semicolon.

let is an expression that expects to have a semicolon on the end so it's definitely needed here at the end of the if/else block

there's more on the possible ways this can go wrong here - https://learning.oreilly.com/library/view/mastering-rust/9781789346572/828f9d43-7c4b-487a-abc2-8689b77c4e17.xhtml - but the gist is that if you don't have a semicolon on the end of the if/else block then you'll get a compiler error.
