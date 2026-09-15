# LOTUS Scripting

[![crates.io](https://img.shields.io/crates/v/lotussim-script.svg)](https://crates.io/crates/lotussim-script)
[![docs.rs](https://img.shields.io/docsrs/lotussim-script)](https://docs.rs/lotussim-script)
![license MIT](https://img.shields.io/badge/LICENSE-MIT-green)
![license APACHE2.0](https://img.shields.io/badge/LICENSE-APACHE2.0-green)
![maintenance active](https://img.shields.io/badge/maintenance-active-green)

Rust-API für Scripts im [LOTUS-Simulator](https://store.steampowered.com/app/370350/LOTUSSimulator/).

Rust API for [LOTUS Simulator](https://store.steampowered.com/app/370350/LOTUSSimulator/) scripts.

## Verwendung / Usage

Als Script-Autor reicht die eine Abhängigkeit `lotussim-script`. Im Rust-Code heißt das Crate `lotus_script`.

As a script author, depend only on `lotussim-script`. In Rust code the crate is named `lotus_script`.

```toml
[dependencies]
lotussim-script = "0.8"
```

Scripts werden nach `wasm32-unknown-unknown` gebaut und laufen in der LOTUS-Scriptengine.

Scripts are compiled for `wasm32-unknown-unknown` and run inside the LOTUS script engine.

```rust
use lotus_script::prelude::*;

#[derive(Default)]
struct MyScript;

impl Script for MyScript {
    fn tick(&mut self) {
        // ...
    }

    fn init(&mut self) {
        // ...
    }

    fn on_message(&mut self, msg: lotus_script::message::Message) {
        // ...
    }
}

script!(MyScript);
```

Der Script-Typ muss `Default` implementieren. `script!` exportiert die WASM-Einstiegspunkte.

The script type must implement `Default`. `script!` exports the WASM entry points.

## Hilfe / Help

- Offline-Hilfe im LOTUS-Simulator
- [LOTUS-Forum](https://www.lotus-simulator.de/forum/)
- API-Dokumentation auf [docs.rs/lotussim-script](https://docs.rs/lotussim-script)

## Suggestions / Contributions

Submit an issue/PR. But in almost all cases it's better to first open
an issue before submitting a PR, so you don't waste your time implementing
a PR which may get rejected.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](https://github.com/Oriolus-Software/LOTUS-Scripting/blob/HEAD/LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](https://github.com/Oriolus-Software/LOTUS-Scripting/blob/HEAD/LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.
