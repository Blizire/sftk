# steamplay

Simple tool/library for starting a steam game on your system. Ideal usage
would be using this as cli tool to just start games from the command line.

Features I want to be included:
- start/stop steam games
- list steam games currently running
- finding steam libraries automatically
- able to be used as a cli tool and as an actually library for future projects

```rust
for entry in fs::read_dir(root)? {
    let entry = entry?;
    let path = entry.path();

    if path.is_file() {
        // Process file
    } else if path.is_dir() {
        // Recurse
    }
}
```

