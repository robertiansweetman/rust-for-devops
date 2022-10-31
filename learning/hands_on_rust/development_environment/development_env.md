# Commands

`cargo new <projectname>` create a new project
`cargo check` see if the project is valid
`cargo build` compile but don't run
`cargo clean` remove the target directory
`cargo run` compile and run in debug mode
`cargo build` compile and run in debug mode
`cargo run --release` compile and run in release mode
`cargo fmt` format code
`cargo clippy` check for common mistakes
`cargo search` look for crates with [search term] then add to Cargo.toml

```
[dependencies]
bracket-lib = {
    git = "gut url here",
    default-features = false,
    features = [ "use-sdl2" ]
}
```

Remove dependencies from your project by deleting them from the cargo.toml file then run `cargo clean`.
