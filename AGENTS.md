# Agent Instructions and Context

This file provides context for AI agents assisting with this project.

## Project Goal

The primary goal is to build a Linux desktop application using Rust and the `gpui` framework to manage Steam's Proton compatibility prefixes.

### Key Features
- List Steam games.
- Inspect Proton prefixes (`compatdata`).
- Run custom `.exe` files within a game's Proton environment.
- Eventually, provide other management features like using `winetricks`.

## Core Technologies

- **Language:** Rust
- **GUI Framework:** `gpui` (from Zed editor)
- **Steam Integration:** `steamlocate` crate to find Steam installations and game libraries.

## Development Best Practices (Rust)

- **Clippy:** Adhere to `clippy::pedantic` and `clippy::nursery` lints where reasonable. Run `cargo clippy` regularly.
- **Formatting:** Always format code using `rustfmt`.
- **Error Handling:** Use `anyhow::Result` for functions that can fail, providing context to errors.
- **Immutability:** Prefer immutable variables and data structures unless mutability is explicitly required.
- **Ownership:** Pass data by reference (`&` or `&mut`) to avoid unnecessary clones and clearly express ownership semantics.
- **Testing:** Add unit and integration tests for core logic.

## Documentation & Reference

- **`gpui` Crate:** The primary documentation is available at [docs.rs/gpui](https://docs.rs/gpui). The official `gpui` repository and its examples are the ultimate source of truth, as the crate is rapidly evolving.
- **`gpui-component` Crate:** This appears to be a third-party crate. Its documentation should be consulted on [docs.rs/gpui-component](https://docs.rs/gpui-component) if used. Be aware it's not part of the official Zed/GPUI ecosystem.
- **Proton Execution:** The logic for running commands within a Proton prefix is complex. Refer to projects like `protontricks` for established methods of setting up the environment (`STEAM_COMPAT_DATA_PATH`, etc.).
