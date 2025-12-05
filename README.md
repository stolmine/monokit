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

Official docs are in progress. For now, there is an extensive (if terse) help system available inside the program.

Press `esc` or `alt + h` to access it. `Ctrl f` will search the help system, use `[ ]` to change sections and up/down arrows to read.

## Configuration

User configuration is stored in `~/.config/monokit/config.toml`.

48 themes are included out of the box. Run `THEMES` to list available themes, or `THEME <name>` to switch.

## License

GPL-2.0

## Acknowledgments

Heavily inspired by the wonderful [monome Teletype](https://monome.org/docs/teletype/).
