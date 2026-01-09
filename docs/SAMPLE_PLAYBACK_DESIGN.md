# Sample Playback System Design

Design document for sample playback voice integration.

**Status:** Planning
**Complexity:** [Very High] - 4-6 weeks
**Phase:** 7 (Advanced DSP)

---

## Overview

Add sample playback as a 6th parallel voice in monokit's multi-synth architecture, with emphasis on slicing capabilities for drum programming and pattern-based sequencing.

---

## Architecture Integration

### Voice Addition: monokit_sampler (Node 1007)

Extends the existing 5-voice architecture:

```
monokit_noise (1000) → Bus 18
monokit_mod (1001) → Bus 17
monokit_primary (1002) → Bus 16
monokit_plaits (1004) → Bus 19/20
monokit_sampler (1007) → Bus 21  [NEW]
    ↓
monokit_main (1006) - Mix all voices + effects
```

### Signal Flow

```
Sample Buffer
    ↓
PlayBuf/BufRd [start/end frames, rate, direction]
    ↓
Envelope [gate or one-shot]
    ↓
Volume
    ↓
Bus 21 → monokit_main
```

### Effects Routing Decision

**Option A: Post-VCA (like Plaits)**
- Samples bypass filter/distortion
- Maintains sample character
- Simpler implementation

**Option B: Pre-VCA (like Complex osc)**
- Samples go through full signal chain
- More sound design options
- Requires careful gain staging

**Recommendation:** Start with Post-VCA, add configurable routing later.

---

## SuperCollider Buffer Management

### Buffer Allocation System

**Requirements:**
- Load WAV/AIFF files via `Buffer.read()`
- Track multiple buffers in memory (bank/slot system)
- Monitor total buffer memory usage
- Free unused buffers to prevent memory leaks

**Structure:**
```
Buffer slots: 0-127 (128 total)
Banks: 0-15 (16 banks of 8 slots each)
Current implementation: Dictionary mapping slot → buffer_id
```

**OSC Commands:**
- `/b_allocRead` - Load sample file to buffer
- `/b_free` - Free buffer memory
- `/b_query` - Get buffer info (frames, channels, sr)

### Memory Management

**Constraints:**
- SuperCollider default: 8MB buffer memory
- Typical 1-second 44.1k stereo sample: ~352KB
- Max samples at default: ~23 one-second stereo samples
- Consider increasing server buffer memory for sample-heavy usage

**Strategy:**
- Lazy loading: only load when S.LOAD called
- Reference counting: free when slot reassigned
- Memory warnings when approaching limits

---

## Slicing System

### Phase 1: Grid-Based Slicing (Priority 1)

**Simple equal division of sample:**

```
S.SLICE <n>       # Divide into N slices (2-64)
S.IDX <0-N>       # Select slice for playback
```

**Implementation:**
```
numSlices = 16
sliceLength = bufferFrames / numSlices
startFrame = sliceLength * sliceIndex
endFrame = startFrame + sliceLength
```

**Pattern Integration:**
```
S.SLICE 16
S.IDX PN.NEXT 0   # Step through slices
S.IDX RND 0 15    # Random slice selection
S.IDX EITH 0 4 8  # Choose from specific slices
```

**Advantages:**
- Simple to implement
- Predictable behavior
- Works well with drum loops
- Full pattern system integration

### Phase 2: Transient Detection (Priority 2)

**Auto-detect slice points based on transients:**

```
S.DETECT          # Analyze and create slice markers
S.THRESH <0-1>    # Detection sensitivity (default 0.5)
```

**Implementation Options:**

**A. Onsets UGen (realtime):**
```supercollider
onsets = Onsets.kr(
    FFT(LocalBuf(512), PlayBuf.ar(1, buf)),
    threshold
)
```
- Pros: Built-in SC functionality
- Cons: Requires FFT, CPU overhead

**B. Pre-analysis (preferred):**
- Analyze buffer on load
- Store slice points in array
- No runtime CPU cost
- More reliable results

**Data Structure:**
```
slicePoints = [0, 4410, 8820, 13230, ...]  // Frame positions
```

**Advantages:**
- Musically intelligent slicing
- Works with unquantized breaks
- Auto-adapts to sample content

**Challenges:**
- False positives (over-slicing)
- Missed transients (under-slicing)
- Parameter tuning required

### Phase 3: Manual Slice Markers (Priority 3)

**User-defined slice points:**

```
S.MARK <time_ms>  # Add slice at position
S.MARK.DEL <idx>  # Delete slice marker
S.MARKS.CLR       # Clear all markers
S.MARKS           # List current markers
```

**Use Cases:**
- Fine-tune auto-detected slices
- Manual slicing of non-transient samples
- Create custom slice arrangements

**Implementation:**
- Store in sample metadata
- Persist with scenes
- Display in UI (future: waveform view?)

---

## SynthDef Implementation

### monokit_sampler SynthDef

```supercollider
SynthDef(\monokit_sampler, {
    arg out = 21,
        bufnum = 0,
        t_gate = 0,
        rate = 1,           // Playback rate (0.1-4)
        startFrame = 0,     // Slice start
        endFrame = -1,      // Slice end (-1 = buffer end)
        direction = 1,      // 1=forward, -1=reverse
        loop = 0,           // 0=one-shot, 1=loop
        volume = 0.5,       // 0-1
        attack = 0.001,     // Envelope attack
        release = 0.01;     // Envelope release

    var sig, env, frames, actualEnd, phasor;

    // Calculate actual end frame
    frames = BufFrames.kr(bufnum);
    actualEnd = Select.kr(endFrame < 0, [endFrame, frames]);

    // Playback engine
    phasor = Phasor.ar(
        trig: t_gate,
        rate: rate * BufRateScale.kr(bufnum) * direction,
        start: startFrame,
        end: actualEnd,
        resetPos: Select.kr(direction > 0, [actualEnd, startFrame])
    );

    // Loop or one-shot
    sig = BufRd.ar(
        numChannels: 1,
        bufnum: bufnum,
        phase: phasor,
        loop: loop,
        interpolation: 4  // Cubic interpolation
    );

    // Envelope
    env = EnvGen.ar(
        Env.asr(attack, 1, release),
        gate: t_gate,
        doneAction: Done.freeSelf
    );

    sig = sig * env * volume;
    Out.ar(out, sig);
}).add;
```

### Trigger Behavior

**Option A: Dedicated trigger (STR)**
```
STR               # Trigger sampler voice only
S.IDX 4           # Select slice
STR               # Play slice 4
```

**Option B: Unified trigger**
```
TR                # Triggers all active voices (Complex, Plaits, Sampler)
S.IDX 4
TR                # Play slice 4 + other voices
```

**Option C: Auto-trigger on index change**
```
S.IDX 4           # Automatically triggers slice 4
```

**Recommendation:** Option A (dedicated STR) for maximum control.

---

## File System & Sample Library

### Directory Structure

```
~/Library/Application Support/monokit/samples/
  ├── drums/
  │   ├── kicks/
  │   │   ├── kick_001.wav
  │   │   ├── kick_002.wav
  │   │   └── kick_003.wav
  │   ├── snares/
  │   └── hats/
  ├── loops/
  │   ├── breaks/
  │   └── bass/
  └── one-shots/
```

### Sample Library Commands

```
S.PATH <path>     # Set sample library root (persistent)
S.BANKS           # List available banks (subdirectories)
S.BANK <0-N>      # Select active bank (or by name)
S.LIST            # List samples in current bank
S.LOAD <name>     # Load sample by name from current bank
S.LOAD <path>     # Load sample by full/relative path
S.INFO            # Show current sample info
```

### File Browser System

**Requirements:**
- Scan directories for WAV/AIFF files
- Build index for quick access
- Support both name-based and numeric selection
- Handle missing files gracefully

**Implementation:**
```rust
struct SampleLibrary {
    root_path: PathBuf,
    banks: Vec<SampleBank>,
    current_bank: usize,
}

struct SampleBank {
    name: String,
    samples: Vec<SampleInfo>,
}

struct SampleInfo {
    name: String,
    path: PathBuf,
    size_bytes: u64,
    // Cached from file header:
    sample_rate: u32,
    channels: u16,
    frames: u64,
}
```

### Path Handling

**Absolute paths:**
```
S.LOAD /Users/name/samples/kick.wav
```

**Relative to library root:**
```
S.PATH ~/samples
S.LOAD drums/kicks/kick_001.wav
```

**Bank/slot system:**
```
S.BANK drums/kicks
S.LOAD kick_001.wav
```

---

## Rust Command Layer

### New Files

```
src/commands/synth/sampler.rs       # Sample playback commands
src/sample_library.rs               # File system & indexing
src/state.rs                        # Add sample state fields
```

### State Additions

```rust
// In AppState or similar
pub struct SampleState {
    pub library: SampleLibrary,
    pub buffers: HashMap<usize, BufferId>,  // slot → SC buffer ID
    pub current_bank: usize,
    pub current_slot: usize,
    pub current_slices: usize,
    pub current_slice_idx: usize,
    pub slice_points: Vec<usize>,  // Frame positions
    pub playback_params: SamplerParams,
}

pub struct SamplerParams {
    pub rate: f32,
    pub direction: i8,
    pub loop_mode: bool,
    pub volume: i32,  // 0-16383
}
```

### Command Implementation

```rust
// S.LOAD command
pub fn handle_sample_load(
    ctx: &mut ExecutionContext,
    path: &str,
    output: &mut Vec<String>,
) -> Result<(), String> {
    // 1. Resolve path (relative to library root)
    // 2. Read WAV header for validation
    // 3. Send /b_allocRead OSC to SC
    // 4. Wait for /done reply
    // 5. Store buffer ID in state
    // 6. Update current slot
    // 7. Reset slice state
}

// S.SLICE command
pub fn handle_sample_slice(
    ctx: &mut ExecutionContext,
    num_slices: i32,
    output: &mut Vec<String>,
) -> Result<(), String> {
    // 1. Validate slice count (2-64)
    // 2. Calculate slice points (equal divisions)
    // 3. Store in state
    // 4. Reset current slice index to 0
}

// S.IDX command (with expression support)
pub fn handle_sample_index(
    ctx: &mut ExecutionContext,
    expr: &str,
    output: &mut Vec<String>,
) -> Result<(), String> {
    // 1. Evaluate expression (supports PN.NEXT, RND, etc.)
    // 2. Validate index < num_slices
    // 3. Calculate start/end frames
    // 4. Send /n_set to update monokit_sampler params
    // 5. Update current_slice_idx in state
}
```

---

## Scene Persistence

### Scene JSON Additions

```json
{
  "sample_state": {
    "current_bank": 0,
    "current_slot": 2,
    "sample_path": "drums/kicks/kick_001.wav",  // Relative path
    "num_slices": 16,
    "current_slice": 4,
    "slice_mode": "grid",  // "grid", "transient", or "manual"
    "slice_points": [0, 2756, 5512, ...],  // For manual/transient
    "playback": {
      "rate": 1.0,
      "direction": 1,
      "loop": false,
      "volume": 8192
    }
  }
}
```

### Loading Strategy

**On scene load:**
1. Check if sample file exists at path
2. If not found, search in current library root
3. If still not found, prompt user or load silently without sample
4. Restore slice configuration
5. Restore playback parameters

**Cross-platform considerations:**
- Store relative paths from library root
- Use platform-agnostic path separators
- Handle missing samples gracefully

---

## Command Reference

### Core Commands

| Command | Range | Description |
|---------|-------|-------------|
| S.LOAD <path> | - | Load sample file |
| S.FREE | - | Free current buffer |
| S.INFO | - | Show sample info |
| STR | - | Trigger sample playback |

### Library Management

| Command | Range | Description |
|---------|-------|-------------|
| S.PATH <path> | - | Set library root |
| S.BANKS | - | List available banks |
| S.BANK <n> | 0-15 | Select bank |
| S.LIST | - | List samples in bank |

### Slicing

| Command | Range | Description |
|---------|-------|-------------|
| S.SLICE <n> | 2-64 | Divide into N slices |
| S.IDX <n> | 0-N | Select slice index |
| S.DETECT | - | Auto-detect transients |
| S.THRESH <n> | 0-16383 | Detection threshold |

### Playback

| Command | Range | Description |
|---------|-------|-------------|
| S.RATE <n> | 0.1-4.0 | Playback rate |
| S.DIR <n> | -1/1 | Direction (fwd/rev) |
| S.LOOP <n> | 0/1 | Loop mode |

---

## Implementation Phases

### Phase 1: Basic Playback [~1 week]

**Goal:** Single sample loading and one-shot playback

- [ ] Create monokit_sampler SynthDef
- [ ] Implement S.LOAD command
- [ ] OSC buffer allocation (/b_allocRead)
- [ ] STR trigger command
- [ ] Basic playback parameters (rate, volume)
- [ ] Single buffer slot only
- [ ] Manual testing with WAV files

**Deliverable:** Can load and trigger a single sample

### Phase 2: Grid Slicing [~1 week]

**Goal:** Equal-division slicing with pattern integration

- [ ] S.SLICE command implementation
- [ ] S.IDX command with expression support
- [ ] Start/end frame calculation
- [ ] Pattern integration (PN.NEXT, RND, EITH)
- [ ] Scene persistence for slice state
- [ ] Test with drum breaks

**Deliverable:** Can slice and sequence samples via patterns

### Phase 3: Banks & Library [~1 week]

**Goal:** Multi-sample management and file browser

- [ ] Sample library directory structure
- [ ] File system scanning and indexing
- [ ] Bank/slot system (128 slots)
- [ ] S.PATH, S.BANKS, S.BANK, S.LIST commands
- [ ] Buffer memory management
- [ ] Sample metadata caching
- [ ] Scene persistence for library state

**Deliverable:** Can organize and browse sample collections

### Phase 4: Advanced Features [~2 weeks]

**Goal:** Professional slicing and playback options

- [ ] Transient detection slicing
- [ ] Manual slice markers
- [ ] S.THRESH parameter tuning
- [ ] Reverse playback (S.DIR -1)
- [ ] Loop mode (S.LOOP 1)
- [ ] Configurable effects routing
- [ ] Waveform visualization (future UI)
- [ ] Sample editor integration

**Deliverable:** Full-featured sample playback system

---

## Integration with Existing Systems

### Pattern System

```
# Slice sequencing
S.SLICE 8
P0 0 1 2 3 4 5 6 7
S.IDX P.NEXT

# Euclidean drum slicing
S.SLICE 16
IF ER 5 8 I: S.IDX I
STR

# Random slice with probability
PROB 75: S.IDX RND 0 15
STR
```

### Variables

```
# Store slice in variable
A S.IDX RND 0 7
S.IDX A
STR

# Conditional slicing
IF GT A 4: S.IDX 0
IF LT A 4: S.IDX 7
```

### Metro & Init Scripts

```
# Metro: Slice progression
M1: S.IDX P.NEXT
M2: STR

# Init: Load default sample
I1: S.LOAD drums/break.wav
I2: S.SLICE 16
```

---

## Technical Challenges

### Buffer Memory Limits

**Problem:** SuperCollider's default buffer memory is limited (8MB)

**Solutions:**
1. Increase server buffer memory in startup config
2. Implement buffer streaming for long samples
3. Add memory usage monitoring and warnings
4. Auto-free unused buffers

### Slice Point Accuracy

**Problem:** Transient detection may produce inconsistent results

**Solutions:**
1. Provide multiple detection algorithms (onset, amplitude, spectral)
2. Allow threshold parameter adjustment
3. Support manual refinement of auto-detected slices
4. Use pre-analysis (not realtime) for better accuracy

### Sample Rate Conversion

**Problem:** Samples may have different sample rates than SC server

**Solutions:**
1. Use BufRateScale.kr for automatic rate compensation
2. Display warning if sample rate differs significantly
3. Option to resample on load (future enhancement)

### Cross-Platform Paths

**Problem:** Sample paths may break across platforms

**Solutions:**
1. Store relative paths from library root
2. Use std::path for platform-agnostic handling
3. Fall back to search if absolute path fails
4. Provide path migration tools

---

## Performance Considerations

### CPU Usage

- PlayBuf: ~1-2% CPU per voice (minimal)
- BufRd + Phasor: ~2-3% CPU per voice
- Transient detection: 5-10% CPU during analysis
- Recommendation: Use PlayBuf for simple cases, BufRd for advanced control

### Memory Usage

- 1-second stereo 44.1k: ~352KB
- 10-second stereo 44.1k: ~3.5MB
- Monitor total buffer allocation
- Warn at 80% of buffer memory limit

### Disk I/O

- Loading samples blocks on file read
- Consider async loading for large samples
- Cache sample metadata to avoid repeated file reads

---

## Future Enhancements

### Sample Editor Integration
- Visual waveform display
- Click-to-set slice points
- Waveform zoom and navigation
- Slice auditioning

### Pitch Shift (Independent of Rate)
- Preserve playback speed while changing pitch
- Requires pitch shifting UGen (PitchShift, not PV_MagSmear)
- Higher CPU cost

### Multi-Channel Support
- Stereo sample playback
- Multi-output routing (kick to separate channel, etc.)

### Sample Recording
- Record input to buffer
- Convert to slice-able sample
- Export to WAV file

### Sample Pool Sharing
- Load sample once, reference in multiple slots
- Reduce memory usage for duplicate samples

---

## Testing Strategy

### Unit Tests
- Buffer allocation/deallocation
- Slice point calculation
- Path resolution
- Expression evaluation in S.IDX

### Integration Tests
- Load → Slice → Trigger workflow
- Pattern integration (P.NEXT, RND)
- Scene save/load with sample state
- Bank switching

### Manual Testing Checklist
- [ ] Load various WAV/AIFF formats (8/16/24-bit, mono/stereo)
- [ ] Slice drum breaks (8, 16, 32 slices)
- [ ] Sequence slices via patterns
- [ ] Test reverse playback
- [ ] Test loop mode
- [ ] Verify memory cleanup on sample change
- [ ] Cross-platform path handling
- [ ] Performance with 10+ loaded samples

---

## Documentation Updates Required

- **MANUAL.md** - Add sample playback section
- **COMMAND_REFERENCE.md** - Add S.* commands
- **ARCHITECTURE.md** - Update voice diagram
- **ROADMAP.md** - Move from Phase 7 to completion
- **CHANGELOG.md** - Document in release notes

---

## Conclusion

Sample playback is a high-impact feature that significantly expands monokit's capabilities from pure synthesis to hybrid synthesis/sampling. The slicing system is the key differentiator - grid-based slicing provides immediate value with manageable complexity, while transient detection enables advanced workflows.

**Estimated total effort: 4-6 weeks**

**Priority phasing:**
1. Basic playback (1 week) - validates architecture
2. Grid slicing (1 week) - unlocks primary use case
3. Library management (1 week) - production-ready
4. Advanced features (2 weeks) - professional polish

The multi-synth architecture makes integration clean: monokit_sampler slots in as Node 1007 alongside existing voices with minimal changes to core systems.
