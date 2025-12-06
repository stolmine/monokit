# Monokit

Teletype-style scripting front end for a complex oscillator voice built in SuperCollider.

Only for M series Macs at the moment.

## Features

### Sound Engine
- **Complex oscillator voice** - complex oscillator configuration with FM, feedback, and waveshaping/folding
- **Full effects chain** - SVF filter, comb resonator, ring mod, bit and sample rate reduction, compressor, beat repeat, pitch shift, stereo delay, 3-band EQ, plate reverb
- **77 real-time parameters** - All controllable via terse commands
- **6 percussive envelopes** - Amp, pitch, FM, discontinuity, feedback, filter with per-envelope attack and curve

### Scripting Language
- **8 script slots** - Each with 8 lines of code, local J/K variables
- **Metro script** - Runs on internal or MIDI clock (rock-solid timing)
- **Init script** - Runs on scene load
- **Pattern system** - 6 patterns × 64 steps with full manipulation (push, pop, rotate, shuffle, scale, etc.)
- **SEQ notation** - Inline sequences with note names, repeats, toggles, random choice
- **Scale quantization** - 12 built-in scales plus custom via bitmask
- **Control flow** - IF/ELIF/ELSE, loops, probability, every-N, skip-N

### Interface
- **TUI with 10 pages** - Live, Scripts 1-8, Metro, Init, Patterns, Variables, Notes, Scope, Help
- **Real-time meters** - Peak/RMS audio levels, 15-band spectrum analyzer
- **Oscilloscope** - Multiple render modes (braille, block, line, dot)
- **Activity indicators** - Visual feedback for script execution and triggers
- **48 color themes** - True color and 256-color fallback support
- **Global search** - Search help pages or scripts with Ctrl+F

### Recording & Sync
- **Direct recording** - 24-bit stereo WAV to current directory
- **MIDI clock sync** - Follow external tempo with sub-millisecond accuracy
- **Scene system** - Save/load complete state (scripts, patterns, parameters)
- **22 factory presets** - Kicks, snares, hats, bass, leads, FX
- **Seamless transitions** - Save/load does not interrupt clock, move between scenes with confidence

### Technical
- **Self-contained bundle** - No SuperCollider installation required
- **~16MB footprint** - Includes scsynth and all required plugins
- **Rust CLI** - Fast startup, low resource usage
- **Configurable audio output** - Choose an audio device from within the program at any time (will restart audio engine)

## Installation

### macOS (Homebrew)

```
brew tap stolmine/monokit
brew install monokit
```

No dependencies required - scsynth audio engine is bundled.

### Building from Source

```bash
cargo build --release --features scsynth-direct
```

Requires Rust 1.70+ and SuperCollider 3.14+ (for bundling scsynth).

## Usage

Run `monokit` on your terminal after installation.

## Documentation

Official docs are in progress. For now, there is an extensive (if terse) help system available inside the program.

Press `esc` or `alt + h` to access it. `Ctrl f` will search the help system, use `[ ]` to change sections and up/down arrows to read.

## Caveat

This software is a TUI (terminal user interface) app. It is designed to run in a 50x18 terminal window.

There is no line length limit, however. The TUI will also scale to whatever window size you choose and zoom attractively with `cmd +/-`.

It is optimized for true color terminals like iTerm2, but will run with a limited color range on the native MacOS terminal (8 bit depth).

I'd suggest exploring `/themes/themes.toml` if you do not like the look of your instance.

Themes and saved scenes can be modified directly under ~/.config/monokit. 

## Configuration

User configuration is stored in `~/.config/monokit/config.toml`.

48 themes are included out of the box. Run `THEMES` to list available themes, or `THEME <name>` to switch.

## License

GPL-2.0 - do what you want if you share!

## Acknowledgments

Heavily inspired by the wonderful [monome Teletype](https://monome.org/docs/teletype/)

And the Industrial Music Electronics [Hertz Donut mk2](https://modulargrid.net/e/industrial-music-electronics-hertz-donut-mk-ii)

Made possible with binaries and sound engine from [SuperCollider](https://supercollider.github.io/)

As well as ugens from [sc3-plugins](https://github.com/supercollider/sc3-plugins)

## Disclosure

Vibe-coded (or not so vibe-coded) with Claude.
