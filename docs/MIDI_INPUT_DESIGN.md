# MIDI Input Design

## Overview

MIDI CC and note input for real-time parameter control. Sequencing takes priority - MIDI sets target values, the slew system handles smooth transitions, and sequencer commands override when active.

## Architecture

### Existing Infrastructure

The slew system already handles target→actual transitions:
```
SendParam(target) → Lag.kr in SuperCollider → smooth transition
```

MIDI CC simply uses this existing path. If sequencer sends a different value, it overwrites the target - **sequencer takes priority** automatically.

### CC Flow

```
src/midi.rs (callback ~line 364)
┌─────────────────────────────────────────────────────────┐
│ MIDI CC (0xBx, cc#, value)                              │
│    ↓                                                    │
│ Lookup: cc_map[cc#] → param_name (e.g., 74 → "fc")     │
│    ↓                                                    │
│ Scale: 0-127 → param's native range (e.g., 0-16383)    │
│    ↓                                                    │
│ metro_tx.send(MetroCommand::SendParam("fc", value))    │
└─────────────────────────────────────────────────────────┘
         ↓ (existing path)
   metro.rs → route_param_to_node() → OSC → SuperCollider
         ↓
   Lag.kr applies slew → smooth transition
```

### Key Files

| Component | File | Lines | Purpose |
|-----------|------|-------|---------|
| MIDI Callback | `src/midi.rs` | 317-365 | CC/note processing hook |
| Parameter Routing | `src/types/synth_types.rs` | 36-70 | Map params to synth nodes |
| Slew System | `src/commands/slew.rs` | 12-101 | Per-param slew times |
| Metro Thread | `src/metro.rs` | 510-523 | SendParam processing |
| Existing MIDI Cmds | `src/commands/system/midi.rs` | 6-87 | MIDI.IN, MIDI.DIAG |

---

## CC Mapping

### Data Structure

```rust
// src/types/midi_types.rs
pub struct MidiCcMap {
    pub mappings: HashMap<u8, String>,  // cc# → param_name
}

impl Default for MidiCcMap {
    fn default() -> Self {
        let mut m = HashMap::new();
        // Sensible defaults
        m.insert(1, "mb".into());     // mod wheel → modbus
        m.insert(74, "fc".into());    // cutoff → filter cut
        m.insert(71, "fq".into());    // resonance → filter q
        m.insert(73, "a".into());     // attack
        m.insert(72, "r".into());     // release
        m.insert(7, "volume".into()); // volume
        Self { mappings: m }
    }
}
```

### Commands

| Command | Description |
|---------|-------------|
| `MIDI.MAP <cc> <param>` | Map CC to parameter |
| `MIDI.UNMAP <cc>` | Remove mapping |
| `MIDI.LEARN <param>` | Next CC received maps to param |
| `MIDI.LIST` | Show current mappings |
| `MIDI.SAVE` | Persist mappings to config |

---

## Note Input

### Options

1. **Trigger scripts**: Note 60-67 → scripts 1-8
2. **Trigger samples**: Note → `STR <slice>`
3. **Set pitch**: Note → `PL.FREQ <midi_note>` for Plaits
4. **Hybrid**: Configurable per-note routing

### Velocity Routing

Velocity could route to:
- Volume/amplitude
- Envelope amount
- Modbus level
- Configurable destination

---

## Design Decisions (TBD)

1. **CC Scaling** - Linear 0-127 → param range, or allow curves?
2. **Pickup Mode** - Should CC "pick up" current value before taking over (prevents jumps)?
3. **Note Velocity** - Route to volume/envelope/modbus?
4. **MIDI Channel** - Filter by channel or accept all?
5. **14-bit CC** - Support CC pairs for high-resolution (matches 14-bit param ranges)?

---

## Implementation Steps

1. Add `MidiCcMap` to config/state
2. Extend MIDI callback to handle CC messages (status 0xBx)
3. Implement CC→param lookup and scaling
4. Add MIDI.MAP/UNMAP/LIST/LEARN commands
5. Add note handling (script triggers or pitch)
6. Persist mappings to config file
7. Update help system

---

## Related Systems

- **ModBus** (`src/commands/synth/modulation.rs`) - Similar target/routing model
- **Slew** (`src/commands/slew.rs`) - Handles smooth transitions
- **SMOOTHABLE_PARAMS** - List of params that support slew (26 currently)
