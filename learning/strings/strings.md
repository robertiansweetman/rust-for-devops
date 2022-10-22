# strings

rust strings are guaranteed to be UTF-8 encoded byte sequences

```rust
let s = "hello world"; // &str

let person: String =  "bob. ".to_string(); // String

let namaste = String::from("नमस्ते"); // unicode string

```
the difference is that 'String' are allocated on the heap where as &str are usually pointers to existing strings on the stack, heap or a string in the data segment of a compiled code object.

The `&` is an operator that is used to create a pointer to any type.

