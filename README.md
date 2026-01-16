# Monokit

```
       _________
      /        /  _  _ ______________________
     /  ______ \________                    /          /
    /        /         /                   /     .    /
   /        /         / ____  ____  ____  /____  ' __//__
  //       //       // /   / /   / /   / //     /   //
          //        / //__/ //  / //__/ / \  __//_  \__
                  //  _  _ ____________//  \
```

![Discord](https://img.shields.io/discord/1446968284225212450?style=flat-square&label=Discord&labelColor=%239D00FF&color=%23FFFFFF)

Teletype-style scripting front end for a suite of several voices: a custom monosynth based on classic complex oscillator arrangements, an implementation of Plaits, and a simple yet effective sample playback engine.

Designed for percussion, glitch, and microsound purposes, but has broad range.

The UX and core logic is written in Rust, the audio components are handled by SuperCollider.

Runs on macOS (Apple Silicon), Linux (x86_64), and Windows (x86_64).

See command_reference.md for quick overview of basic options.

## Disclosure

Vibe-coded (whether the vibes are good or bad who can say) with Claude Code.

That being the case, I have tried as much as possible to approach things with care and I have put a great deal of thought, intention, and design into this software.

Hopefully it shows!

## Features

### Sound Engine
- **Complex oscillator voice** - dual oscillators with selectable waveforms + white/pink/brown noise, independent volume per source
- **Extensive modulation routing** - mod osc → primary freq/mod freq/filter cutoff/filter Q/discontinuity/VCA; noise → primary FM/mod FM
- **Plaits macro oscillator** - 16 synthesis engines (VA, FM, wavetable, granular, percussion, physical modeling)
- **Sample Playback** - Kit/slice modes with dynamic and instant file handling, 14-type filter, decimator, MiRings resonator (samples excite MiRings as a resonator)
- **Full effects chain** - 14-type multimode filter, ring mod, lo-fi (bit/sample rate), compressor, beat repeat, pitch shift, stereo delay, 3-band EQ, plate reverb (Plaits is inserted after main voice VCA and is not affected by filter, ring mod, or lofi) + an implementation of Clouds
- **97 real-time parameters** - All instantly controllable via terse commands
- **Per-stage envelopes** - Amp, pitch, FM, discontinuity, feedback, filter, noise, modbus - most with independent attack, decay, and curve
- **Slew control** - 26 voice/fx parameters with 0-10000ms range, each independently settable
- **VCA modes** - Gate mode for percussion, drone mode for sustained tones; noise has independent gating

### Scripting Language
- **Flexible scripting** - Hundreds of unique commands for controlling sequencing, synthesis, and UI at runtime. Most take expressions as arguments for further complexity.
- **8 script slots** - Each with 8 lines of code
- **Script mutes** - Individual mute toggles for each script (Ctrl+Shift+1-8/M/I)
- **Metro script** - Runs on internal or MIDI clock
- **Init script** - Runs on scene load
- **Variables** - 8 global variables, 4 incrementing accumulators, 2 local vars per script, several specialized variables for use with particular commands
- **Pattern system** - 6 patterns × 64 steps with full manipulation (push, pop, rotate, shuffle, scale, etc.)
- **SEQ notation** - Inline sequences with note names, raw values, repeats, toggles, random choice
- **Control flow** - IF/ELIF/ELSE, loops, probability, every-N, skip-N

### Interface
- **TUI with 10 pages** - Live, Scripts 1-8, Metro, Init, Patterns, Variables, Notes, Scope, Help
- **Page navigation** - Programmatic page switching via PAGE/PG commands
- **Real-time meters** - Peak/RMS audio levels, 15-band spectrum analyzer
- **Oscilloscope** - Multiple render modes (braille, block, line, dot)
- **Activity indicators** - Visual feedback for script execution and triggers
- **48 color themes** - True color and 256-color fallback support
- **Global search** - Search help pages or scripts with Ctrl+F
- **Quick quit** - Ctrl+Q to quit instantly from anywhere

### Recording & Sync
- **Direct recording** - 24-bit stereo WAV to current working directory
- **Configurable audio output** - Choose an audio device from within the program at any time (will restart audio engine i.e. short downtime)
- **Solid timing core** - real time thread priority and tuned sleep system produce steady clock with on-grid output
- **MIDI clock sync** - Follow external tempo with high accuracy (requires a couple beats on start to sync)
- **Scene system** - Save/load complete state (scripts, patterns, parameters), scenes can be called via script to create song structures or continuous performances
- **Seamless transitions** - Save/load is instant and does not interrupt clock, move between scenes with confidence
- **22 factory presets** - Kicks, snares, hats, bass, leads, FX, save any script as a preset and insert it into a script on command, functionally infinite storage

### Data Driven Musicality
- **Quantization** - Snap primary and modulator osc frequency to a range of preset scales, or use binary notation to create microtonal scales over any division of the octave
- **Note selection** - N op automatically maps osc frequency to chromatic semitone values or to bitmasked setting

### Lightweight Binary
- **Self-contained bundle** - No SuperCollider installation required
- **~17MB footprint** - Includes scsynth and all required plugins
- **Rust CLI** - Fast startup, low resource usage

## Installation

### macOS (Homebrew)

```bash
brew tap stolmine/monokit
brew install monokit
```

No dependencies required - scsynth audio engine is bundled.

### Linux (AppImage)

Download the AppImage from the [latest release](https://github.com/stolmine/monokit/releases/latest):

```bash
chmod +x Monokit-*-x86_64.AppImage
./Monokit-*-x86_64.AppImage
```

Single portable file, no installation needed. Requires PipeWire or JACK for audio.

### Linux (Tarball)

```bash
tar -xzf monokit-*-x86_64-unknown-linux-gnu.tar.gz
cd monokit-*-x86_64-unknown-linux-gnu
./monokit
```

### Windows

Download the ZIP from the [latest release](https://github.com/stolmine/monokit/releases/latest):

1. Extract `monokit-*-x86_64-pc-windows-msvc.zip`
2. Open the extracted folder
3. Run `monokit.exe`

Config is stored in `%APPDATA%\monokit\` (persists across runs).

**Note:** For low-latency audio on Windows, ASIO drivers are recommended (e.g., ASIO4ALL or your audio interface's native ASIO driver).

### Building from Source

```bash
cargo build --release --features scsynth-direct
```

Requires Rust 1.70+ and SuperCollider 3.14+ (for bundling scsynth).

## Usage

Run `monokit` on your terminal after installation.

## Documentation

Official docs are in progress. For now, there is an extensive (if terse) help system available inside the program.

Press `esc` or `alt + h` to access it. `Ctrl f` will search the help system, use `[ ]` (or `Ctrl+[` / `Ctrl+]`) to change sections and up/down arrows to read.

## Caveat

This software is a TUI (terminal user interface) app. It is designed to run in a 50x18 terminal window.

There is no line length limit, however. The TUI will also scale to whatever size you choose and zoom attractively with `cmd +/-`. iTerm2 handles this beautifully.

Monokit is optimized for true color terminals like iTerm2, but will run with a limited color range on the native MacOS terminal (8 bit depth, 256 colors).

I'd suggest exploring `/themes/themes.toml` if you do not like the look of your instance. Themes are easily customizable and take simple hex values for all fields.

## Configuration

User configuration is stored in platform-specific locations:
- **macOS:** `~/Library/Application Support/monokit/config.toml`
- **Linux:** `~/.config/monokit/config.toml`
- **Windows:** `%APPDATA%\monokit\config.toml`

48 themes are included out of the box. Run `THEMES` to list available themes, or `THEME <name>` to switch.

## License

GPL-2.0 - do what you want if you share!

## Acknowledgments

Heavily inspired by the wonderful [monome Teletype](https://monome.org/docs/teletype/)

And the Industrial Music Electronics [Hertz Donut mk2](https://modulargrid.net/e/industrial-music-electronics-hertz-donut-mk-ii)

Includes [Plaits](https://pichenettes.github.io/mutable-instruments-documentation/modules/plaits/), [Clouds](https://pichenettes.github.io/mutable-instruments-documentation/modules/clouds/), and [Rings](https://pichenettes.github.io/mutable-instruments-documentation/modules/rings/) by Émilie Gillet, praise be! SuperCollider ports via [mi-UGens](https://github.com/v7b1/mi-UGens) by Volker Böhm.

Made possible with binaries and sound engine from [SuperCollider](https://supercollider.github.io/)

As well as UGens from [sc3-plugins](https://github.com/supercollider/sc3-plugins) (GPL-2.0):
- **BlackrainUGens** (SVF, BMoog filters) by blackrain
- **TJUGens** (DFM1 diode filter) by Tony Hardie-Bick & Jonny Stutters

Keyboard smash title text animation based on this repo from [DvorakDwarf](https://github.com/DvorakDwarf/scrambling-title-animations?tab=readme-ov-file)

ASCII logo by CiDE

No official association with any authors of the aforementioned is implied or actual. DO NOT direct support requests to these people, direct them to this repository or to the discord.
