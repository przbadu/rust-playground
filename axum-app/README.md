### How to start

```sh
cd axum-basic
cargo build
cargo run
```

**Or watch the changes**

Install cargo-watch

```sh
cargo install cargo-watch
```

```sh
cd axum-basic
cargo watch -q -c -w src/ -x run
```

Visit http://localhost:8000 to see it in action

### How to test

**Watch the test**

```sh
cd axum-basic
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```
