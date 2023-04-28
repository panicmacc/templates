## About

A Nix Flake template for an `mdbook` project.

## Usage

- `nix develop`  activates a dev environment, and should put `mdbook` CLI tool in your PATH.
- A book skeleton is already generated.
- The book source is in `src/`.
- `mdbook serve` starts a hot-reloading dev server.
- `mdbook build` will put static content in `book/`.
- `.gitignore` ignores `book/`.

## Refs

[Introduction - mdBook Documentation](https://rust-lang.github.io/mdBook/)
