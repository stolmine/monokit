# Monokit Documentation Index

Quick reference and links to all project documentation.

---

## Quick Links

| Document | Description | Lines |
|----------|-------------|-------|
| [ROADMAP.md](ROADMAP.md) | Current priorities, v0.4.12 complete | ~395 |
| [CHANGELOG.md](CHANGELOG.md) | Version history, recent updates | ~477 |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Project structure, key files | ~180 |
| [MANUAL.md](MANUAL.md) | User manual with tutorials | ~1615 |

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
| [PLAITS_INTEGRATION.md](PLAITS_INTEGRATION.md) | Plaits voice implementation |
| [DSP_TIER1_IMPLEMENTATION_PLAN.md](../DSP_TIER1_IMPLEMENTATION_PLAN.md) | Filter, Resonator, Delay, Reverb |
| [EFFECT_ROUTING_DESIGN.md](EFFECT_ROUTING_DESIGN.md) | Effect routing system |
| [MIDI_CLOCK_TIMING_LESSONS.md](MIDI_CLOCK_TIMING_LESSONS.md) | MIDI clock diagnostics |
| [DEBUG_TIERS.md](../DEBUG_TIERS.md) | Debug tier classification system |
| [TIER_0_COMMANDS.md](../TIER_0_COMMANDS.md) | Tier 0 (silent) command analysis |
| [TIER_FIXES_SUMMARY.md](../TIER_FIXES_SUMMARY.md) | Debug tier fixes summary |

### Release & Distribution
| Document | Description |
|----------|-------------|
| [RELEASE_PIPELINE.md](RELEASE_PIPELINE.md) | Automated release workflow |
| [HOMEBREW_BUNDLE_FORMULA.md](HOMEBREW_BUNDLE_FORMULA.md) | Homebrew formula docs |
| [BUNDLE_QUICK_START.md](BUNDLE_QUICK_START.md) | Bundle quick start |

---

## Current Status (v0.4.12 - Complete)

| Feature | Status |
|---------|--------|
| Beat Repeat/Pitch Shift Short Aliases | **DONE** |
| Envelope Parameter Tier Fixes | **DONE** |
| ExecutionContext Test Updates | **DONE** |
| Dynamic Bundle Size in Release | **DONE** |
| CPU Readout Fixed Width | **DONE** |

### v0.4.11 (Complete)
| Feature | Status |
|---------|--------|
| MFF/MFQ Filter Modulation Amounts | **DONE** |
| Ctrl+Shift+Z Redo Fix | **DONE** |
| Deprecated Noise Params Removal | **DONE** |
| Debug Tier Refactor | **DONE** |
| ExecutionContext Refactor | **DONE** |

### v0.4.0 (Complete)
| Feature | Status |
|---------|--------|
| Plaits Pitch Control (PL.FREQ/PLF) | **DONE** |
| 3-Letter Plaits Aliases | **DONE** |
| PLTR Trigger Readout | **DONE** |
| Multi-Voice Trigger Indicators (H\|P → C\|P) | **DONE** |
| RND.PL Fixes | **DONE** |

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
- Noise: nw, np, nm, nv
- Plaits: pitch, harmonics, timbre, morph, engine, decay, lpg, plv, pav
- Source Levels: pv, mv
- Envelopes: ad, pd, fd, dd, pa, fa, da
- Lo-Fi: lb, ls, lm
- Filter: fc, fq, ft (0-13), fe, fed, fk, mc, mq, mff, mfq
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
