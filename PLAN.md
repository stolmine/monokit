# Monokit UI Refactor Plan - Teletype-Style Interface

## Overview

Refactor monokit's TUI to follow teletype's sparse, page-based design with script storage and pattern system.

## Pre-Implementation Tasks

- [ ] Add `.gitignore` with `teletype-main/` and `target/`

## Key Teletype Concepts to Adopt

### 1. Page/Mode System

**Teletype has:**
- LIVE mode (command input + sub-modes for vars, grid, dash)
- EDIT mode (script editing for 1-8, M, I)
- PATTERN mode (4 patterns × 64 steps)
- PRESET modes (load/save)
- HELP mode (17 pages)

**Monokit adaptation:**
- **LIVE** (REPL) - Terminal-style command execution, output scrolls
- **SCRIPT 1-8** - 8 script pages, each stores lines committed to memory
- **M** (Metro) - Metro script page
- **I** (Init) - Init script page (runs on startup)
- **PATTERN** - Pattern storage/editing (4 patterns × 64 steps)
- **HELP** - Command reference

Navigation: `[` and `]` cycle through pages

### 2. Script Storage Model

**Per script:**
- **8 lines per script** (slightly more than teletype's 6)
- Lines stored as text, parsed on execution
- Each script has local J, K variables

**Script types:**
- Scripts 1-8: Triggered manually or via future trigger inputs
- M script: Runs on each metro tick (replaces current M: syntax)
- I script: Runs once on startup

**Script page behavior:**
- Shows script name in header
- Lists stored lines (numbered 1-8)
- Input region at bottom for editing/adding lines

**Line editing keybindings:**
- **Up/Down** - Navigate between stored lines (selected line loads into input)
- **Enter** - Save input to current line (or next empty slot if no selection)
- **Shift+Enter** - Duplicate current line to next line
- **Delete/Backspace** - Delete one character at cursor
- **Shift+Delete** - Delete entire line
- **Alt+Left/Right** - Move cursor word-by-word
- **Left/Right** - Move cursor character-by-character

**Editing flow:**
1. Navigate to a line with Up/Down → line contents appear in input region
2. Edit the line in input region
3. Press Enter to save changes
4. Or press Down to move to next line (discards unsaved changes)

**Persistence: Manual Save/Load**
- `SAVE <name>` - save current scene (all scripts + patterns) to file
- `LOAD <name>` - load scene from file
- Scene files stored in `~/.monokit/scenes/` as JSON or text

### 3. Pattern System

**Structure:**
- 4 patterns (0-3), selected via `P.N`
- 64 steps per pattern
- Each step is int16 value
- Playhead with idx, len, wrap, start, end

**Operations to implement:**
- `P` / `PN` - get/set value at index
- `P.L` / `PN.L` - pattern length
- `P.I` / `PN.I` - playhead index
- `P.N` - working pattern selector
- `P.NEXT` / `P.PREV` - advance/reverse playhead
- `P.HERE` - value at current position

### 4. Variables

**Core variables (persist during session):**
- `A`, `B`, `C`, `D` - general accumulators
- `X`, `Y`, `Z`, `T` - general accumulators

**Per-script variables:**
- `I` - loop counter (execution scoped)
- `J`, `K` - per-script counters

### 5. Control Flow & PRE Separator

**PRE separator (`:`):**
```
IF X > 5: PF 200
L 0 7: P I; TR
PROB 50: DC 8000
```
- Left of `:` is condition/modifier
- Right of `:` executes if condition passes

**Control flow ops (all implemented):**
- `IF <cond>:` - conditional execution
- `ELIF <cond>:` - else-if (must follow IF/ELIF)
- `ELSE:` - fallback (must follow IF/ELIF)
- `L <start> <end>:` - loop with I as iterator (inclusive)
- `PROB <0-100>:` - probabilistic execution (percentage)
- `EVERY <n>:` - execute every nth trigger
- `SKIP <n>:` - skip every nth trigger (inverse of EVERY)

**Comparison operators:**
- `>`, `<`, `>=`, `<=`, `==`, `!=`
- Can compare variables and literals

**Sub-command separator (`;`):**
- Multiple commands on one line: `PF 100; DC 8000; TR`
- Executes left to right

### 6. Sparse UI Design

**Principles from teletype:**
- Fixed 8-line display area (we have more, but keep it minimal)
- No scrollbars - explicit pagination
- Keyboard-only navigation
- Dirty region refresh (only update changed areas)
- Minimal decoration - content focused

**Layout:**
```
┌─ MONOKIT ─────────────────────────────┐
│ LIVE  1  2  3  4  5  6  7  8  M  I  P │  <- Page tabs
├───────────────────────────────────────┤
│                                       │
│  (page content - script lines or      │
│   REPL output or pattern data)        │
│                                       │
├───────────────────────────────────────┤
│ > input                               │
└───────────────────────────────────────┘
```

## Implementation Phases

### Phase 1: Page Infrastructure
- Add Page enum variants: Live, Script1-8, Metro, Init, Pattern, Help
- Implement `[` / `]` navigation across all pages
- Add page indicator in header

### Phase 2: Script Pages
- Create ScriptStorage struct (lines, J, K per script)
- Implement script page rendering (show stored lines)
- Script input: Enter adds line to script, not executes
- Add line selection and deletion

### Phase 3: Live Page (REPL)
- Keep current terminal-style behavior
- Commands execute immediately
- Output scrolls like terminal
- Can call `SCRIPT n` to execute stored script

### Phase 4: Pattern System
- Add pattern storage (4 × 64 int16)
- Implement P ops in command parser
- Pattern page shows values in columns
- Pattern page allows editing values

### Phase 5: Variables & Control Flow
- Add variable storage (A-D, X-Z, T, I, J, K)
- Implement variable get/set ops
- Add PRE separator parsing
- Implement IF, L, PROB control flow

### Phase 6: Script Execution
- Parse stored script lines on trigger
- Execute M script on metro tick
- Execute I script on startup
- SCRIPT command to call numbered scripts

## Files to Modify

- `src/main.rs` - Page system, UI rendering, script storage
- `Cargo.toml` - No new deps needed
- `sc/monokit_server.scd` - No changes

## Decisions Made

- **8 lines per script** (slightly more than teletype's 6)
- **Manual save/load** - SAVE/LOAD commands for scene persistence
- **Full control flow** - IF, ELIF, ELSE, L, PROB, EVERY, SKIP from start

## Implementation Order

1. **Gitignore** - Add teletype-main/ and target/
2. **Page system** - Expand pages, fix navigation
3. **Script storage** - Data structures for 10 scripts × 8 lines
4. **Script page UI** - Rendering and line editing
5. **Variables** - A-D, X-Z, T, I, J, K storage and ops
6. **PRE separator** - Parser changes for `:` and `;`
7. **Control flow** - IF/ELIF/ELSE, L, PROB, EVERY/SKIP
8. **Pattern storage** - 4 × 64 int16 arrays
9. **Pattern ops** - P, PN, P.L, P.I, P.N, P.NEXT, etc.
10. **Pattern page UI** - Display and editing
11. **Script execution** - SCRIPT command, M script on metro, I on init
12. **Save/Load** - Scene serialization to ~/.monokit/scenes/
