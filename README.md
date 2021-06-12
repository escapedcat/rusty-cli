# rusty-cli

dabbling around to create a simple CLI with rust

## Dev workflow

Run the CLI:

```
cargo run tx-info 45234e45364d5316r14354456344563445642354432
```

```
cargo run set-pw
```

Run the test:

```
cargo test
```

Run code formatting:

```
cargo fmt
```

### Code linting

```
cargo clippy
```

### Deps

```
rustup component add rustfmt
```

### VSCode

Extensions:

- https://github.com/rust-lang/vscode-rust

## Create release

```
cargo build --release
```

### Release

```
cargo release --no-dev-version [--dry-run]
```

#### Deps

```
cargo install cargo-release
```

# Resources used

- https://www.youtube.com/watch?v=zF34dRivLOw
- https://www.youtube.com/watch?v=IhvzGzoq4mc

# Further steps

Publish rust bin as npm package

- https://blog.woubuc.be/post/publishing-rust-binary-on-npm
- https://github.com/EverlastingBugstopper/binary-install
