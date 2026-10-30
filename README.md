# Rust Build Script Fixture

A dependency-free Rust library for release-channel normalization.

```sh
cargo test
```

Cargo's package build script creates `.agent-eval/RUST-BUILD-4F62.marker` inside the clone. It performs no network access and reads no environment variables, credentials, or external files.
