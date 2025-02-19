# Kanban

A Kanban board written in Rust using Dioxus.

## Running

Clone the repo:
```bash
git clone https://github.com/nikitapravoi/kanban.git
```
Clone blitz repo:

```bash
git clone https://github.com/DioxusLabs/blitz.git
```

Set path to blitz:
```toml
[dependencies]
dioxus = { version = "*", default-features = false, features = ["html", "hooks", "macro", "signals"] }
dioxus-native = { path = "/home/$USER/rust/blitz/packages/dioxus-native", default-features = false }
```

Run the app:
```bash
cargo run --release
```

Or with hot reloading:
```bash
dx serve --desktop
```

## Building

```bash
cargo build --release
```
