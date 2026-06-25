# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is a Cargo workspace containing three demo binaries that each showcase a different git backend for the [`vergen`](https://github.com/rustyhorde/vergen) build-info injection library:

- `vergen-git2-demo` — uses `vergen-git2` (libgit2 bindings)
- `vergen-gitcl-demo` — uses `vergen-gitcl` (shell `git` CLI)
- `vergen-gix-demo` — uses `vergen-gix` (pure-Rust `gix`)

All three demos do the same thing at runtime: print build metadata (build timestamp, cargo info, git info, rustc version, sysinfo) via `vergen-pretty`.

## Commands

```sh
# Build all workspace members
cargo build

# Build a single demo
cargo build -p vergen-gix-demo

# Run a demo (triggers build.rs, which fetches git metadata)
cargo run -p vergen-gix-demo
cargo run -p vergen-gitcl-demo
cargo run -p vergen-git2-demo

# Check without building binaries
cargo check

# Clippy (full lint set only activates on nightly)
cargo +nightly clippy
```

There are no tests in this workspace — these crates exist solely as integration demos.

## Architecture

### Build pipeline

Each crate's `build.rs` uses the respective vergen library to emit Cargo build instructions at compile time. The emitter chain is identical across all three:

```
Build::all_build() + Cargo (normal deps only) + Git backend + Rustc::all_rustc() + Sysinfo::all_sysinfo() + Custom entry
```

The custom entry (`AddCustomEntries` impl) inserts a `vergen-cl = "custom_instruction"` env var as an example of the extension point.

All build scripts fetch from the remote `https://github.com/rustyhorde/vergen-cl.git` at tag `0.3.9` to demonstrate remote git operations. `git2` and `gitcl` variants pass a `fetch_depth` (200 and 300 respectively); `gix` omits it.

### Runtime

`main.rs` is identical across all three crates: it calls `vergen_pretty_env!("vergen-cl")` to collect all env vars emitted by the build script, then renders them with color via `vergen-pretty`.

### Lint configuration

All `main.rs` files carry a large `#![cfg_attr(nightly, deny(...))]` block. Lints are gated behind the `nightly` cfg flag (set by `rustversion` in `build.rs`). The `unstable` feature flag enables additional experimental nightly lints.

### Local development against vergen source

`Cargo.toml` contains a commented-out `[patch.crates-io]` section for pointing at local checkouts of `vergen-gix`, `vergen-gitcl`, and `vergen-git2`. Uncomment and adjust paths when testing against unreleased vergen changes.
