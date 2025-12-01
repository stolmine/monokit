# MIDI Clock Timing: Lessons Learned

This document captures the debugging journey and solutions for achieving stable MIDI clock sync timing between Monokit (Rust), OSC, and SuperCollider.

## The Problem

When syncing to external MIDI clock (e.g., from Ableton via IAC Bus), audio output exhibited a "galloping" or "swing" effect - alternating short (~102ms) and long (~214ms) intervals instead of steady 148ms intervals at 101 BPM.

## Diagnostic Infrastructure Built

### Rust Side (MIDI.DIAG)
- `MIDI.DIAG 1` - Enable timing diagnostics
- `MIDI.DIAG 0` - Disable diagnostics
- `MIDI.DIAG REPORT` - Write report to `midi_timing_report.txt`

Reports include:
- 16th note interval statistics (mean, std dev, min, max, jitter %)
- Raw 24 PPQN pulse analysis
- Swing detection (odd vs even interval comparison)
- Metro thread timing
- Trigger count

### SuperCollider Side (SC.DIAG)
- `SC.DIAG 1` - Enable SC timing diagnostics
- `SC.DIAG 0` - Disable diagnostics
- `SC.DIAG REPORT` - Write report to `sc_timing_report.txt`

Reports include:
- Trigger count received
- Interval statistics
- Swing analysis with ratio calculation

## Root Causes Discovered

### Issue 1: UDP Packet Loss (10% of messages dropped)
**Symptom**: Rust sent 261 triggers, SC received only 234.

**Cause**: Default UDP socket buffer sizes too small, causing packet loss even on localhost.

**Solution**: Use `socket2` crate to set 4MB send buffer:
```rust
const OSC_BUFFER_SIZE: usize = 4 * 1024 * 1024;

let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
socket.set_send_buffer_size(OSC_BUFFER_SIZE)?;
```

**Result**: 165 sent = 165 received. No more packet loss.

### Issue 2: UI Event Loop Blocking (THE MAIN FIX)
**Symptom**: Even with no packet loss, SC showed alternating ~102ms/~214ms intervals.

**Cause**: The UI event loop used `event::poll(Duration::from_millis(100))` which blocked metro event processing for up to 100ms while waiting for keyboard input.

**The Critical Fix**:
```rust
// BEFORE (broken):
if event::poll(Duration::from_millis(100))? {

// AFTER (fixed):
if event::poll(Duration::from_millis(1))? {
```

**Result**: SC intervals became consistent 145-150ms (jitter dropped from 160ms to 4ms).

### Issue 3: SC Gate Timing (Secondary)
**Symptom**: Triggers received but not all produced audible sound.

**Cause**: Using `s.makeBundle(0.01, ...)` for gate=0 without proper latency offset caused race conditions.

**Solution**: Use simple `SystemClock.sched` for gate release:
```supercollider
~voice.set(\gate, 1);
SystemClock.sched(0.01, {
    ~voice.set(\gate, 0);
    nil;
});
```

## Final Architecture

```
MIDI Clock (Ableton)
    ↓ 24 PPQN via IAC Bus
Monokit (Rust)
    - midir receives MIDI clock
    - Counts 6 pulses = 1 sixteenth note
    - 4MB UDP send buffer (socket2)
    - Real-time thread priority (audio_thread_priority)
    - Spin sleep for precise timing (spin_sleep)
    ↓ OSC via UDP (immediate messages in MIDI mode)
SuperCollider
    - OSCdef receives /monokit/trigger
    - Immediate gate=1, SystemClock.sched gate=0
    - No s.latency buffering needed
```

## Key Learnings

1. **UDP packet loss happens on localhost** - Always set large socket buffers (4MB+) for reliable OSC.

2. **UI event loops can destroy timing** - Never block the main loop for more than 1ms when timing-critical events need processing.

3. **Diagnostic infrastructure is essential** - Building counters on both ends (triggers sent vs received) immediately revealed the packet loss issue.

4. **Timestamps at each stage** - Recording timestamps at MIDI input, metro thread, and SC reception pinpointed exactly where timing degraded.

5. **Don't add latency unnecessarily** - In MIDI sync mode, messages should be immediate. Adding `s.latency` buffering in SC made things worse.

6. **Simple is better for triggers** - Complex OSC bundle timing wasn't needed. Simple immediate messages + fast event loop = perfect timing.

## Performance Achieved

| Metric | Before | After |
|--------|--------|-------|
| Packet loss | 10% | 0% |
| SC interval jitter | 160ms | 4ms |
| Swing ratio | 1.048 | 0.9997 |
| Processing latency | N/A | <1μs |

## Files Modified

- `Cargo.toml` - Added socket2 dependency
- `src/metro.rs` - 4MB socket buffer, timing diagnostics
- `src/midi.rs` - MIDI timing stats, pulse analysis
- `src/ui/mod.rs` - 1ms event poll timeout (THE FIX)
- `src/commands/system/midi.rs` - MIDI.DIAG command
- `src/commands/system/sc.rs` - SC.DIAG command (new)
- `sc/monokit_server.scd` - SC-side diagnostics
