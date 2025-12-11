# Monokit Documentation Index

Quick reference and links to all project documentation.

---

## Quick Links

| Document | Description | Lines |
|----------|-------------|-------|
| [ROADMAP.md](ROADMAP.md) | Current priorities, v0.3.4 progress | ~160 |
| [CHANGELOG.md](CHANGELOG.md) | Version history, recent updates | ~150 |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Project structure, key files | ~180 |
| [COMMAND_REFERENCE.md](COMMAND_REFERENCE.md) | Complete command reference | ~400 |
| [MANUAL.md](MANUAL.md) | User manual with tutorials | - |

### History Archive
| Document | Description |
|----------|-------------|
| [history/CHANGELOG.md](history/CHANGELOG.md) | Detailed completion records |
| [history/PHASES.md](history/PHASES.md) | Development phases 1-6 |
| [history/FUTURE.md](history/FUTURE.md) | Phase 7-8 future plans |

### Technical Specifications
| Document | Description |
|----------|-------------|
| [scsynth_direct_integration.md](scsynth_direct_integration.md) | Direct scsynth integration |
| [DSP_TIER1_IMPLEMENTATION_PLAN.md](DSP_TIER1_IMPLEMENTATION_PLAN.md) | Filter, Resonator, Delay, Reverb |
| [EFFECT_ROUTING_DESIGN.md](EFFECT_ROUTING_DESIGN.md) | Effect routing system |
| [MIDI_CLOCK_TIMING_LESSONS.md](MIDI_CLOCK_TIMING_LESSONS.md) | MIDI clock diagnostics |

### Release & Distribution
| Document | Description |
|----------|-------------|
| [RELEASE_PIPELINE.md](RELEASE_PIPELINE.md) | Automated release workflow |
| [HOMEBREW_BUNDLE_FORMULA.md](HOMEBREW_BUNDLE_FORMULA.md) | Homebrew formula docs |
| [BUNDLE_QUICK_START.md](BUNDLE_QUICK_START.md) | Bundle quick start |

---

## Current Status (v0.3.4)

| Feature | Status |
|---------|--------|
| Config Flag Standardization | **DONE** |
| SCRIPT Expression Support | **DONE** |
| MC/MQ Filter Routing | **DONE** |
| ModBus Envelope (MBA/MBD) | **DONE** |
| Script Undo/Redo | **DONE** |
| ER/NR Rhythm Operators | **DONE** |
| Additional Filter Types | **DONE** |
| Noise Source | Not started |
| Oscillator Sync | Not started |

---

## OSC Protocol

All communication from Rust CLI to SuperCollider server uses UDP (127.0.0.1:57120).

**Message Format:**
- **Trigger:** `/monokit/trigger` (no arguments)
- **Master Volume:** `/monokit/volume` (float 0.0-1.0)
- **Parameter Control:** `/monokit/param <name> <value>`
- **Reset:** `/monokit/reset` (no arguments)
- **Recording:** `/monokit/rec`, `/monokit/rec/stop`, `/monokit/rec/path`

**Parameter Names:**
- Oscillator/FM: pf, pw, mf, mw, fb, fba, fbd, dc, dm, dd, tk, mb, mp, md, mt, ma, fm, mx, mm, me
- Envelopes: ad, pd, fd, dd, pa, fa, da
- Lo-Fi: lb, ls, lm
- Filter: fc, fq, ft, fe, fed, fk, mf_f, mf_q
- Ring Mod: rgf, rgw, rgm
- Resonator: rf, rd, rm, rk
- Compressor: ct, cr, ca, cl, cm
- Pan: pan
- Beat Repeat: br_act, br_len, br_rev, br_win, br_mix
- Pitch Shift: ps_mode, ps_semi, ps_grain, ps_mix, ps_targ
- Delay: dt, df, dlp, dw, ds, dmode, dtail
- EQ: el, em, ef, eq, eh
- Reverb: rv, rp, rh, rw, rmode, rtail

---

## Dependencies

### Rust
- rosc 0.10 - OSC protocol
- ratatui 0.29 - Terminal UI framework
- crossterm 0.28 - Terminal backend
- rand 0.8 - Random number generation
- anyhow 1 - Error handling
- thiserror 1 - Error types

### SuperCollider
- SuperCollider 3.x with scsynth

---

## Project Metadata

### License
GPL-2.0 (GNU General Public License v2.0)

### Release Process
- Release script: `scripts/release.sh`
- Automates version tagging and build process
- Creates GitHub releases with compiled binaries

### Distribution
- Homebrew tap: `brew tap stolmine/monokit`
- Formula: `homebrew-monokit/Formula/monokit.rb`
- Install location: `/opt/homebrew/libexec/monokit/`
- Bundle structure: monokit binary + Resources/ (scsynth, plugins, synthdefs) + Frameworks/ (dylibs)
- User config: `~/.config/monokit/`

---

## Document Schema

This documentation set follows a modular structure to keep files under agent read limits (~500 lines ideal):

```
docs/
├── documentation_index.md  # This file - quick reference and links
├── ROADMAP.md              # Current priorities, active development
├── CHANGELOG.md            # Version history, recent updates
├── ARCHITECTURE.md         # Project structure, key files
├── COMMAND_REFERENCE.md    # Complete command reference
├── MANUAL.md               # User manual
├── history/
│   ├── CHANGELOG.md        # Detailed completion records
│   ├── PHASES.md           # Development phases 1-6
│   └── FUTURE.md           # Phase 7-8 future plans
└── [technical specs]       # Implementation plans
```

**Conventions:**
- ROADMAP.md: Active development priorities only
- CHANGELOG.md: Last 3-4 versions, link to history for older
- ARCHITECTURE.md: Project structure, not detailed specs
- COMMAND_REFERENCE.md: Commands only, no version history
- history/: Archive for completed phases and detailed records
