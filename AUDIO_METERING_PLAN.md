# Phase 5.5: Audio Metering Implementation Plan

## Overview

Real-time audio metering with bidirectional OSC (SuperCollider sends meter data to Rust).

## Layout

REC indicator moves to border, meters in header content row:

```
┌─ MONOKIT ─────────────────────── ● REC 02:15 ──┐
│ [LIVE] 1 2 3 4 5 6 7 8 M I P V TR  L▅▆ R▅▅     │
└─────────────────────────────────────────────────┘
```

- REC: top border, right-aligned (ratatui `Title` with `Position::Top`, `Alignment::Right`)
- Meters: 2-char bargraph per channel using `▁▂▃▄▅▆▇█`
- Clip: changes meter color to error

## Architecture

```
SuperCollider ──SendPeakRMS──> UDP 57121 ──> Meter Thread ──channel──> App
                (20Hz)                        (src/meter.rs)
```

## Data Structures

```rust
// src/types.rs

#[derive(Debug, Clone, Default)]
pub struct MeterData {
    pub peak_l: f32,      // 0.0 - 1.0
    pub peak_r: f32,
    pub rms_l: f32,
    pub rms_r: f32,
    pub peak_hold_l: f32, // Decays slowly
    pub peak_hold_r: f32,
    pub clip_l: bool,
    pub clip_r: bool,
}

// Extend MetroEvent enum:
pub enum MetroEvent {
    ExecuteScript(usize),
    ExecuteDelayed(String, usize),
    MeterUpdate(MeterData),  // NEW
}
```

## SuperCollider Changes

Add to `sc/monokit_server.scd` after final `Out.ar`:

```supercollider
// Send peak/RMS at 20Hz to Rust meter thread
SendPeakRMS.kr(sigL, 20, 3, '/monokit/meter', 1);
SendPeakRMS.kr(sigR, 20, 3, '/monokit/meter', 2);
```

Note: `SendPeakRMS` sends to the address that sent the last message by default,
or we configure explicit reply address.

## Meter Thread

New file `src/meter.rs`:

- Bind UDP socket on port 57121
- Parse `/monokit/meter` OSC messages: `[channel, peak, rms]`
- Update `MeterData` with peak hold decay logic
- Send `MetroEvent::MeterUpdate` to main thread

## Bargraph Rendering

```rust
const BARGRAPH_CHARS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

fn level_to_bar(level: f32) -> String {
    // 2-char display for 0.0-1.0
    let idx = (level * 8.0).min(7.0) as usize;
    format!("{}{}", BARGRAPH_CHARS[idx.min(7)],
            if level > 0.5 { BARGRAPH_CHARS[((level - 0.5) * 16.0).min(7.0) as usize] } else { ' ' })
}
```

## Files to Modify

| File | Change |
|------|--------|
| `sc/monokit_server.scd` | Add `SendPeakRMS` UGens |
| `src/meter.rs` | NEW: receiver thread |
| `src/main.rs` | Add `mod meter`, spawn thread |
| `src/types.rs` | Add `MeterData`, extend `MetroEvent` |
| `src/app/mod.rs` | Add `meter_data: MeterData` field |
| `src/ui/mod.rs` | Handle `MeterUpdate` event |
| `src/ui/header.rs` | Move REC to border title, add meter display |

## Implementation Order

1. Add `MeterData` type and extend `MetroEvent`
2. Create `src/meter.rs` with receiver thread
3. Spawn meter thread in `main.rs`
4. Handle `MeterUpdate` in UI event loop
5. Update `header.rs`: REC to border, add meter spans
6. Modify SC SynthDef with `SendPeakRMS`
7. Test end-to-end

## Optional Enhancements

- **CPU monitoring**: Parse SC `/status` reply for CPU percentage
- **Peak hold markers**: Separate indicator that decays slowly
- **Clip reset**: Click or command to reset clip indicators
