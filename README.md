# Rust starter

[![Deploy on velixir](https://velixir.net/img/deploy-on-velixir.svg)](https://velixir.net/new?template=rust-http)

No tokio, no axum, no crates at all: a `TcpListener` and a hand-written response. That keeps
the release build to seconds rather than minutes, which is the point of a starter.

[Deploy it on velixir](https://velixir.net/new?template=rust-http).

## Running it locally

```bash
cargo run
```

Then open http://localhost:8080.

## Deploying

```bash
velixir deploy
```

velixir runs the release build on the build node, so a cold compile of a heavy dependency
tree never ties up your laptop.

## The one rule

Read `PORT` from the environment and bind `0.0.0.0`. velixir injects `PORT`; a hardcoded or
loopback-only listener builds fine and then fails its health check.

## Reaching for a framework

This deliberately has no dependencies. When you want routing, add `axum` and `tokio` to
`Cargo.toml` as usual: the build node resolves them, and nothing about the deploy changes.

## Licence

MIT.
