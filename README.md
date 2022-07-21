# petourno
World's first n-tuple (triple elimination and up) tournament generator. PWA designed for modularity.

## Configuration

Modify the players and tuple of elimination in `tournament/src/bin/main.rs`. Start with a string of comma-separated players, then choose the tuple by editing the final number.

```rust
// Example (3 player quadruple elimination tournament)
elimination::new_elim("Alice, Bob, Charlie".split(",").map(|p| p.to_owned()).collect::<Vec<String>>(), 4);
```

## Building

```bash
cargo run --bin main --release
```

## Running Web Server

```bash
cargo install trunk
cd frontend
trunk serve --release
```
