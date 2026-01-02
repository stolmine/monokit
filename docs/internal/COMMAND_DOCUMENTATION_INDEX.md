# MONOKIT COMMAND DOCUMENTATION INDEX

Complete audit and inventory of all monokit command debug output handling patterns.

Generated: 2026-01-02  
Scope: 63 command files, 316 debug output calls

---

## Quick Navigation

### For Quick Overview
Start here: **[COMMAND_OUTPUT_SUMMARY.md](COMMAND_OUTPUT_SUMMARY.md)**
- Executive summary of findings
- 4-page overview with priorities
- High-level statistics
- Recommended approach & phases

### For Detailed Catalog
Reference: **[COMMAND_INVENTORY.md](COMMAND_INVENTORY.md)**
- Complete breakdown by category
- Every file with current status
- Line numbers and specific details
- Migration status for each
- 6+ pages of detailed info

### For Technical Details
Reference: **[COMMAND_PATTERNS_DETAIL.md](COMMAND_PATTERNS_DETAIL.md)**
- Code pattern examples
- Tier constants mapping
- Handler signature variations
- Output callback types
- Validation checklist
- Migration roadmap

### For Quick Reference
Reference: **[COMMAND_MATRIX.md](COMMAND_MATRIX.md)**
- All files in matrix format
- Pattern, count, status, priority
- Statistics by category
- Handler function counts
- Magic number locations
- Recommended migration order

---

## Key Findings

### Pattern Distribution
- **NEW** (ctx.output): 4 files, 118 uses (modern)
- **OLD** (debug_level >=): 21 files, 198 uses (legacy)
- **MIXED**: 1 file, 56 uses (partial)
- **MAGIC NUMBERS** (>= 2): 4 files, 27 uses (problematic)
- **NO OUTPUT**: 38 files (silent)

### Overall Status
- 6% fully migrated to new pattern
- 33% still using legacy pattern
- 60% produce no output
- 27 hardcoded magic number comparisons

### Migration Effort
- Target: 223 uses to migrate
- High priority: 172 uses across 14 files
- Medium priority: 98 uses across 9 files
- Low priority: 2 uses in 1 file

---

## Document Purposes

### COMMAND_OUTPUT_SUMMARY.md (Start Here)
**Purpose**: Executive briefing on current state and recommended actions

**Contents**:
- Overview and key finding
- Quick stats table
- Pattern breakdown with examples
- Priority migration matrix
- Magic number problem explanation
- Inconsistencies found
- Recommended approach (4 phases)
- Success criteria

**Best For**: Decision makers, project planning, quick understanding

**Length**: ~340 lines

---

### COMMAND_INVENTORY.md (Detailed Reference)
**Purpose**: Complete catalog of every command file and its status

**Contents**:
- Summary statistics
- Adoption status table
- Detailed breakdown by category:
  - Core Commands (5 files)
  - Pattern Commands (3 files)
  - System Commands (6 files)
  - Synth Commands (many)
  - Gate & Randomization (3 files)
  - Root level utilities
- Migration summary table
- Key findings and status indicators
- Line numbers and specific examples

**Best For**: Detailed auditing, understanding specific files, verification

**Length**: ~327 lines (6+ pages)

---

### COMMAND_PATTERNS_DETAIL.md (Technical Reference)
**Purpose**: Deep technical details of patterns and migration approach

**Contents**:
- Pattern examples with actual code:
  - NEW pattern example
  - OLD pattern example
  - Magic number example
  - Mixed pattern example
- Tier constants mapping
- Output callback types
- Handler signature variations
- Dispatcher call patterns
- Silent file list (38 files)
- Migration roadmap (8 phases)
- Validation checklist for migration

**Best For**: Implementation, code migration, understanding mechanics

**Length**: ~325 lines

---

### COMMAND_MATRIX.md (Quick Reference)
**Purpose**: Tabular overview of all command files and attributes

**Contents**:
- Complete file matrix (all 63 files)
- Statistics by pattern type
- Completion status breakdown
- Priority breakdown
- Total command uses
- Handler function counts
- Magic number occurrences with locations
- Output category usage
- Tier constant usage
- Dependencies graph
- Recommended migration order

**Best For**: Quick lookup, finding specific files, statistics

**Length**: ~234 lines

---

## How to Use These Documents

### I need to understand the scope
1. Read: COMMAND_OUTPUT_SUMMARY.md (Executive section)
2. Review: COMMAND_MATRIX.md (Statistics section)
3. Time: ~10 minutes

### I need to plan the migration
1. Read: COMMAND_OUTPUT_SUMMARY.md (Priority & Approach sections)
2. Reference: COMMAND_PATTERNS_DETAIL.md (Roadmap section)
3. Review: COMMAND_MATRIX.md (Recommended migration order)
4. Time: ~20 minutes

### I need to implement a migration
1. Find your file: COMMAND_INVENTORY.md
2. Get code examples: COMMAND_PATTERNS_DETAIL.md
3. Use checklist: COMMAND_PATTERNS_DETAIL.md (Validation section)
4. Verify in matrix: COMMAND_MATRIX.md
5. Time: Varies by complexity

### I need to find a specific command
1. Search: COMMAND_INVENTORY.md (by category or filename)
2. See context: COMMAND_PATTERNS_DETAIL.md (pattern examples)
3. Track priority: COMMAND_MATRIX.md (priority column)
4. Time: ~5 minutes

### I need to understand inconsistencies
1. Read: COMMAND_OUTPUT_SUMMARY.md (Inconsistencies section)
2. See examples: COMMAND_PATTERNS_DETAIL.md (Pattern examples)
3. Track usage: COMMAND_INVENTORY.md (by file details)
4. Time: ~15 minutes

---

## Key Statistics

### By Category
- Core Commands: 5 files (variables, counters, scheduling, sync, scale)
- Pattern Commands: 3 files (working, explicit, macros)
- System Commands: 7 files (metro, misc, preset, scene, audio, midi, sc)
- Synth Commands: 26 files (common, effects, envelopes, oscillators, filters, etc.)
- Gate & Randomization: 3 files (gate, randomization, slew)
- Utilities: 5 files (validate, context, common, aliases, validate_expr)
- Infrastructure: 9 files (various supporting modules)

### By Output Pattern
- Files with output: 25
- Files without output: 38
- Total files: 63

### Total Output Calls
- NEW pattern: 118 (ctx.output)
- OLD pattern: 198 (debug_level >=)
- MIXED: 56 (both patterns)
- MAGIC NUMBERS: 27 (hardcoded >= 2)
- **TOTAL: 316 debug output calls**

### By Priority (to migrate)
- HIGH: 172 uses (14 files)
- MEDIUM: 98 uses (9 files)
- LOW: 2 uses (1 file)
- DONE/NO OUTPUT: 44 files

---

## Migration Phases Overview

### Phase 1: Quick Wins (2-3 weeks)
- Magic number fixes: 27 uses
- Simple handlers (sync, slew): 12 uses

### Phase 2: Core (3-4 weeks)
- Counters & scheduling: 18 uses
- System config: 16 uses

### Phase 3: Large Refactors (4-6 weeks)
- Synth commands: 42 uses
- Randomization: 25 uses

### Phase 4: Final (2-3 weeks)
- Pattern macros: 59 uses
- System misc: 24 uses

**Total Target**: 223 uses to migrate (21 files)

---

## Files & Their Roles

| Document | Primary Use | Length | Read Time |
|----------|------------|--------|-----------|
| COMMAND_OUTPUT_SUMMARY.md | Executive briefing | 340 lines | 10 min |
| COMMAND_INVENTORY.md | Detailed audit | 327 lines | 20 min |
| COMMAND_PATTERNS_DETAIL.md | Technical guide | 325 lines | 25 min |
| COMMAND_MATRIX.md | Quick lookup | 234 lines | 10 min |
| COMMAND_DOCUMENTATION_INDEX.md | Navigation guide | (this) | 5 min |

**Total Reading**: ~70 minutes for complete understanding  
**Total Lines**: 1,226 lines of documentation

---

## Related Source Files

For reference when implementing:

- `src/commands/context.rs` - ExecutionContext definition
- `src/types/mod.rs` - OutputCategory enum and TIER constants
- `src/commands/mod.rs` - Main dispatcher (189 debug_level references)
- `src/commands/core/variables.rs` - Example of NEW pattern (COMPLETE)
- `src/commands/patterns/working.rs` - Example of NEW pattern (COMPLETE)
- `src/commands/core/counters.rs` - Example of OLD pattern (NEEDS MIGRATION)
- `src/commands/randomization.rs` - Largest OLD pattern user (25 uses)

---

## Quick Links to Key Sections

### In COMMAND_OUTPUT_SUMMARY.md
- [Quick Stats](#quick-stats)
- [Pattern Examples](#pattern-breakdown)
- [Priority Matrix](#priority-migration-matrix)
- [Magic Number Problem](#magic-number-problem)
- [Inconsistencies](#inconsistencies-found)
- [Recommended Approach](#recommended-approach)

### In COMMAND_INVENTORY.md
- [Core Commands](#core-commands)
- [Pattern Commands](#pattern-commands)
- [System Commands](#system-commands)
- [Synth Commands](#synth-commands)
- [Migration Summary](#migration-summary-table)

### In COMMAND_PATTERNS_DETAIL.md
- [NEW Pattern Example](#new-pattern-ctxoutputcategoryxmessage-mut-output)
- [OLD Pattern Example](#old-pattern-if-debug_level--tier_x--output)
- [Magic Number Pattern](#magic-number-pattern-debug_level--2)
- [Tier Mapping](#tier-constants-mapping)
- [Migration Checklist](#validation-checklist-for-migration)
- [Roadmap](#migration-roadmap)

### In COMMAND_MATRIX.md
- [Adoption Status Table](#adoption-status-by-file)
- [Statistics](#statistics)
- [Priority Files](#statistics)
- [Migration Order](#recommended-migration-order)

---

## Document Maintenance

**Last Updated**: 2026-01-02

When migration progresses:
1. Update priority in COMMAND_SUMMARY.md
2. Move files from OLD to NEW in COMMAND_INVENTORY.md
3. Update phase completion in COMMAND_PATTERNS_DETAIL.md
4. Mark files as DONE in COMMAND_MATRIX.md

---

## Summary

These documents provide:
- Complete inventory of all command output handling
- Clear identification of modern vs legacy patterns
- Specific line numbers and code examples
- Priority ranking for migration
- Technical guidance for implementation
- Quick reference tables for all files
- Validation checklists and best practices

**Total Scope**: 63 files, 316 output calls, 223 to migrate
**Current Status**: 6% complete (4 files, 118 uses)
**Target**: 100% migration to NEW pattern

