# Monokit

Teletype-style scripting front end for a SuperCollider complex oscillator voice.

## Requirements

- Rust 1.70+
- SuperCollider 3.13+

## Installation

```bash
cargo build --release
```

## Usage

```bash
./target/release/monokit
```

## Documentation

In progress!

## Configuration

User configuration is stored in `~/.monokit/config.toml`.

48 themes are included out of the box. Run `THEMES` to list available themes, or `THEME <name>` to switch.

## License

GPL-2.0

## Acknowledgments

Heavily inspired by [monome Teletype](https://monome.org/docs/teletype/).
