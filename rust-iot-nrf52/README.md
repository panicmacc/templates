# rust-iot-nrf52 (template)

## How to use

Make a project repo, and clone it to a directory:

```bash
git clone <project_uri> my_proj
cd my_proj
```

Initialize the project flake from template:

```bash
nix flake init -t "git+ssh://git@gitlab.com/rootlife/apps/deck#templates.rust-iot-nrf52"
```

Update project name, description, etc. in `flake.nix` and `Cargo.toml`:

```bash
nvim Cargo.toml flake.nix
```

Update flake dependencies:

```bash
nix flake update
```

Review and stage added/updated files, commit and push:

```bash
git status
git add .
git commit -m "initial project structure"
git push
```

## Develop

From the project directory, use `nix develop` to enter a dev environment with build tools:

```bash
nix develop
```

From the `develop` nix environment, and with a board connected via USB, build, flash, and run with `cargo embed`:

```bash
cargo embed
```

If successful this will open a terminal attached to the MCU RTT debug interface via USB.

## Resources

https://github.com/nrf-rs/nrf-hal/blob/master/examples/hello-world/src/main.rs

https://docs.rs/nrf52833-hal/latest/nrf52833_hal/

https://docs.rs/microbit-v2/0.13.0/microbit/gpio/index.html

https://www.nordicsemi.com/products/nrf52833

https://doc.rust-lang.org/cargo/reference/cargo-targets.html
