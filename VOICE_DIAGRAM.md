# Monokit Voice Architecture

```
                            OSCILLATORS
    ┌─────────────────────────────────────────────────────────┐
    │                                                         │
    │   ┌─────────┐                       ┌─────────┐        │
    │   │  NOISE  │──NP──┐       ┌──NM───│  NOISE  │        │
    │   │ W/P/B   │      │       │        │  ENV    │        │
    │   │  (NW)   │      ▼       ▼        │NA/ND/NC │        │
    │   └────┬────┘   ┌──────────────┐    └─────────┘        │
    │        │        │   PRIMARY    │                        │
    │        │   ┌───▶│  OSC (PW)    │◀── PITCH ENV ──┐      │
    │        │   │    │  Sin/Tri/Saw │    (PA/PD)     │      │
    │        │   │    └──────┬───────┘                │      │
    │        │   │           │                        │      │
    │        │   │           │ FM (FM+FA/FD)          │      │
    │        │   │           │                        │      │
    │        │   │    ┌──────┴───────┐                │      │
    │        │   │    │  MODULATOR   │                │      │
    │   NV───┼───┼───▶│  OSC (MW)    │◀── MD ────────┐│      │
    │        │   │    │Sin/Tri/Saw/FB│    (mod freq) ││      │
    │        │   │    └──────┬───────┘               ││      │
    │        │   │           │                       ││      │
    │        │   │           ▼                       ││      │
    │        │   │    ┌─────────────┐               ││      │
    │        │   │    │   MODBUS    │───────────────┘│      │
    │        │   │    │  (MB/MBA)   │────MP──────────┘      │
    │        │   │    └──────┬──────┘                       │
    │        │   │           │ MA (amp mod)                  │
    │        │   │           │ MC (filter cutoff mod)        │
    │        │   │           │ MQ (filter Q mod)             │
    │        │   │           ▼                               │
    └───────┼───┼───────────┼────────────────────────────────┘
            │   │           │
            ▼   │           ▼
    ┌───────────┴───────────────────┐
    │         SOURCE MIX            │
    │   PV (primary) + MV (mod)     │
    │       + NV (noise)            │
    │     + gain compensation       │
    └───────────┬───────────────────┘
                │
                ▼
    ┌───────────────────────────────┐
    │       DISCONTINUITY           │
    │     DC + (dcEnv * DA/DD)      │
    │  DM: fold/tanh/soft/hard/     │
    │      asym/rect/crush          │
    └───────────┬───────────────────┘
                │
                ▼
    ┌───────────────────────────────┐
    │          LO-FI                │
    │   LB (bits) / LS (rate)       │
    │        LM (mix)               │
    └───────────┬───────────────────┘
                │
                ▼
    ┌───────────────────────────────┐
    │     MULTI-MODE FILTER         │
    │  FC + FK (tracking) + FE/FED  │
    │      + MC (modbus→cutoff)     │
    │  FQ + MQ (modbus→resonance)   │
    │  ─────────────────────────────│
    │  FT 0-3:  SVF LP/HP/BP/Notch  │
    │  FT 4:    MoogFF (ladder LP)  │
    │  FT 5-6:  RLPF/RHPF (12dB)    │
    │  FT 7-8:  DFM1 LP/HP (diode)  │
    │  FT 9-11: BMoog LP/HP/BP      │
    │  FT 12-13: Latch-SC LP/HP     │
    └───────────┬───────────────────┘
                │
                ▼
    ┌───────────────────────────────┐
    │        RING MOD               │
    │  RGF (freq) / RGW (wave)      │
    │       RGM (mix)               │
    └───────────┬───────────────────┘
                │
                ▼
    ┌───────────────────────────────┐
    │     COMB RESONATOR            │
    │   RF + RK (tracking)          │
    │   RD (decay) / RM (mix)       │
    └───────────┬───────────────────┘
                │
                ▼
    ┌───────────────────────────────┐
    │           VCA                 │
    │   AMP ENV (AD) * VOLUME       │
    │   + MA (modbus→amp)           │
    │   VCA mode: gate/drone        │
    └───────────┬───────────────────┘
                │
                ▼
    ┌───────────────────────────────┐
    │        COMPRESSOR             │
    │  CT (thresh) / CR (ratio)     │
    │  CA (attack) / CL (release)   │
    │       CM (makeup)             │
    └───────────┬───────────────────┘
                │
                ▼
    ┌───────────────────────────────┐
    │           PAN                 │
    │          (PN)                 │
    └───────────┬───────────────────┘
                │
         ┌──────┴──────┐
         ▼             ▼
    ┌─────────┐   ┌─────────┐
    │    L    │   │    R    │
    └────┬────┘   └────┬────┘
         │             │
         ▼             ▼
    ┌───────────────────────────────┐
    │       BEAT REPEAT             │
    │  BR.LEN / BR.REV / BR.WIN     │
    │        BR.MIX                 │
    └───────────┬───────────────────┘
                │
                ▼
    ┌───────────────────────────────┐
    │       PITCH SHIFT             │
    │  PS.MODE (gran/freq)          │
    │  PS.SEMI / PS.GRAIN           │
    │  PS.MIX / PS.TARG             │
    └───────────┬───────────────────┘
                │
                ▼
    ┌───────────────────────────────┐
    │       STEREO DELAY            │
    │  DT (time) / DF (feedback)    │
    │  DLP (filter) / DW (wet)      │
    │  DMODE: bypass/insert/send    │
    │  DTAIL: cut/ring/freeze       │
    └───────────┬───────────────────┘
                │
                ▼
    ┌───────────────────────────────┐
    │        3-BAND EQ              │
    │  EL (low) / EM (mid)          │
    │  EF (mid freq) / EQ (mid Q)   │
    │       EH (high)               │
    └───────────┬───────────────────┘
                │
                ▼
    ┌───────────────────────────────┐
    │       PLATE REVERB            │
    │  RV (size) / RP (predelay)    │
    │  RH (damp) / RW (wet)         │
    │  RMODE: bypass/insert/send    │
    │  RTAIL: cut/ring/freeze       │
    └───────────┬───────────────────┘
                │
                ▼
    ┌───────────────────────────────┐
    │     LIMITER / DC BLOCK        │
    │      LIMIT (on/off)           │
    └───────────┬───────────────────┘
                │
                ▼
            ┌───────┐
            │  OUT  │
            │ L / R │
            └───────┘
```

## Parameter Reference

### Oscillators
| Param | Range | Description |
|-------|-------|-------------|
| PF | 20-20000 | Primary frequency (Hz) |
| PW | 0-2 | Primary waveform (sin/tri/saw) |
| MF | 20-20000 | Modulator frequency (Hz) |
| MW | 0-3 | Mod waveform (sin/tri/saw/fb) |
| FM | 0-16383 | FM amount |
| FB | 0-16383 | Feedback (MW=3 only) |
| TK | 0-16383 | Mod freq tracking |
| MT | multiplier | Tracking multiplier |

### Noise Source
| Param | Range | Description |
|-------|-------|-------------|
| NW | 0-2 | Noise type (white/pink/brown) |
| NA | ms | Noise env attack |
| ND | ms | Noise env decay |
| NC | -8 to 8 | Noise env curve |
| NE | 0-16383 | Noise env amount |
| NP | 0-16383 | Noise→primary FM |
| NM | 0-16383 | Noise→mod FM |
| NV | 0-16383 | Noise output level |
| NG | 0-1 | Noise gate (drone/gated) |

### Source Levels
| Param | Range | Description |
|-------|-------|-------------|
| PV | 0-16383 | Primary volume |
| MV | 0-16383 | Modulator volume |

### ModBus
| Param | Range | Description |
|-------|-------|-------------|
| MB | 0-16383 | ModBus depth |
| MBA | 0-16383 | ModBus env amount |
| MBD | ms | ModBus env decay |
| MP | 0-1 | Route to primary freq |
| MD | 0-1 | Route to mod freq |
| MA | 0-1 | Route to amplitude |
| MC | 0-1 | Route to filter cutoff |
| MQ | 0-1 | Route to filter Q |

### Envelopes
| Param | Range | Description |
|-------|-------|-------------|
| AD | ms | Amp decay |
| PD | ms | Pitch decay |
| PA | 0-16383 | Pitch env amount |
| FD | ms | FM decay |
| FA | 0-16383 | FM env amount |
| DD | ms | Disc decay |
| DA | 0-16383 | Disc env amount |
| FBD | ms | Feedback decay |
| FBA | 0-16383 | Feedback env amount |
| FED | ms | Filter env decay |
| FE | 0-16383 | Filter env amount |

### Filter
| Param | Range | Description |
|-------|-------|-------------|
| FC | 20-20000 | Cutoff frequency |
| FQ | 0-16383 | Resonance |
| FT | 0-13 | Filter type |
| FK | 0-16383 | Key tracking |

### Effects
See COMMAND_REFERENCE.md for full effect parameters.
