# v0.4.2 Manual Testing Agenda

**Version:** v0.4.2 "Workflow Enhancement"
**Date:** December 2025
**Tester:** _______________

---

## Pre-Test Setup

### Environment
- [ ] Build monokit from main branch: `cargo build --release`
- [ ] Verify version shows v0.4.2 (run `./target/release/monokit` and check)
- [ ] Clean config for fresh testing: backup and remove `~/.config/monokit/config.toml`
- [ ] Clean scenes for fresh testing: backup `~/Library/Application Support/monokit/scenes/`
- [ ] Ensure SuperCollider is running or scsynth bundle is available

### Verification
- [ ] Launch monokit successfully
- [ ] Audio output working
- [ ] No startup errors in console

---

## Feature 1: Script Mutes

### Test 1.1: Basic Mute via Command

**Setup:**
1. Navigate to Script 1 page (Alt+1)
2. Enter line: `PRINT HELLO FROM SCRIPT 1`
3. Save script

**Test Steps:**
1. From Live page, execute: `SCRIPT 1`
2. Verify output: "HELLO FROM SCRIPT 1" appears
3. Execute: `MUTE 1`
4. Verify output: "SCRIPT 1: MUTED" appears
5. Execute: `SCRIPT 1` again
6. Verify output: "SCRIPT 1 IS MUTED" (script doesn't execute)
7. Execute: `MUTE 1` again (toggle off)
8. Verify output: "SCRIPT 1: ACTIVE"
9. Execute: `SCRIPT 1`
10. Verify output: "HELLO FROM SCRIPT 1" appears again

**Expected:** Mute toggles prevent/allow script execution ✅ / ❌

---

### Test 1.2: Mute via Hotkey

**Setup:**
1. Script 1 still has `PRINT HELLO FROM SCRIPT 1`
2. Script is currently active (unmuted)

**Test Steps:**
1. Press Alt+Shift+1
2. Verify output: "SCRIPT 1: MUTED"
3. Navigate to Script 1 page (Alt+1)
4. Verify title shows: " SCRIPT 1 [MUTED] "
5. Press Alt+Shift+1 again
6. Verify output: "SCRIPT 1: ACTIVE"
7. Verify title shows: " SCRIPT 1 " (no [MUTED])

**Expected:** Hotkey toggles work and UI reflects mute state ✅ / ❌

---

### Test 1.3: Direct Mute Commands

**Setup:**
1. Create test content in scripts 2, 3, Metro (M), and Init (I):
   - Script 2: `PRINT SCRIPT 2`
   - Script 3: `PRINT SCRIPT 3`
   - Metro: `PRINT METRO TICK`
   - Init: `PRINT INIT EXECUTED`

**Test Steps:**
1. Execute: `MUTE.2`
2. Verify: "SCRIPT 2: MUTED"
3. Execute: `MUTE.3 1` (explicit set to muted)
4. Verify: "SCRIPT 3: MUTED"
5. Execute: `MUTE.M`
6. Verify: "SCRIPT M: MUTED"
7. Execute: `MUTE.I 0` (explicit set to active)
8. Verify: "SCRIPT I: ACTIVE"

**Expected:** Direct commands work with optional explicit values ✅ / ❌

---

### Test 1.4: Mute Query Command

**Setup:**
1. Scripts 1, 2, 3, M currently muted
2. Scripts 4-8, I currently active

**Test Steps:**
1. Execute: `MUTE` (no arguments)
2. Verify output shows all 10 scripts with their status:
   ```
   SCRIPT MUTES:
     1: MUTED
     2: MUTED
     3: MUTED
     4: ACTIVE
     5: ACTIVE
     6: ACTIVE
     7: ACTIVE
     8: ACTIVE
     M: MUTED
     I: ACTIVE
   ```

**Expected:** Query command lists all script mute states ✅ / ❌

---

### Test 1.5: Mute with Expression

**Setup:**
1. Script 1 is active

**Test Steps:**
1. Execute: `MUTE 1 GT 2 3` (should evaluate to 0, set active)
2. Verify: "SCRIPT 1: ACTIVE"
3. Execute: `MUTE 1 LT 5 3` (should evaluate to 1, set muted)
4. Verify: "SCRIPT 1: MUTED"

**Expected:** Expression evaluation works in MUTE command ✅ / ❌

---

### Test 1.6: Metro Execution While Muted

**Setup:**
1. Metro script contains: `PRINT TICK`
2. Metro is muted (MUTE.M)
3. Start metro: `M.START 120` (120 BPM)

**Test Steps:**
1. Wait 2-3 seconds
2. Verify: No "TICK" messages appear
3. Unmute metro: `MUTE.M`
4. Verify: "SCRIPT M: ACTIVE"
5. Wait 2-3 seconds
6. Verify: "TICK" messages appear regularly
7. Stop metro: `M.STOP`

**Expected:** Muted metro doesn't execute, unmuted metro executes ✅ / ❌

---

### Test 1.7: Init Mute on Startup

**Setup:**
1. Init script contains: `PRINT INIT RAN`
2. Save scene: `SAVE test_init_mute`
3. Mute init: `MUTE.I`
4. Quit monokit

**Test Steps:**
1. Restart monokit
2. Load scene: `LOAD test_init_mute`
3. Verify: No "INIT RAN" message (init was muted in scene)
4. Unmute: `MUTE.I`
5. Execute: `SCRIPT I`
6. Verify: "INIT RAN" appears

**Expected:** Init mute persists in scenes ✅ / ❌

---

### Test 1.8: Scene Persistence

**Setup:**
1. Mute scripts 1, 3, 5, M
2. Leave scripts 2, 4, 6, 7, 8, I active

**Test Steps:**
1. Save scene: `SAVE test_mutes_persist`
2. Unmute all scripts (toggle each one)
3. Verify all are active
4. Load scene: `LOAD test_mutes_persist`
5. Execute: `MUTE` (query)
6. Verify: Scripts 1, 3, 5, M show MUTED; others show ACTIVE

**Expected:** Mute states restore correctly from scenes ✅ / ❌

---

### Test 1.9: All Hotkey Combinations

**Test Steps:**
1. Press Alt+Shift+1 through Alt+Shift+8 in sequence
2. Verify each outputs "SCRIPT N: MUTED" where N=1-8
3. Navigate to each script page (Alt+1 through Alt+8)
4. Verify each shows "[MUTED]" in title
5. Press Alt+Shift+M
6. Verify: "SCRIPT M: MUTED"
7. Navigate to Metro page (Alt+M)
8. Verify title shows " METRO [MUTED] "
9. Press Alt+Shift+I
10. Verify: "SCRIPT I: MUTED"
11. Navigate to Init page (Alt+I)
12. Verify title shows " INIT [MUTED] "

**Expected:** All hotkeys work and UI updates correctly ✅ / ❌

---

## Feature 2: Page Navigation Commands

### Test 2.1: Basic Page Switching

**Test Steps:**
1. From Live page, execute: `PAGE 1`
2. Verify: Page switches to Script 1, output "SWITCHED TO SCRIPT 1"
3. Execute: `PAGE 5`
4. Verify: Page switches to Script 5, output "SWITCHED TO SCRIPT 5"
5. Execute: `PAGE M`
6. Verify: Page switches to Metro, output "SWITCHED TO METRO"
7. Execute: `PAGE I`
8. Verify: Page switches to Init, output "SWITCHED TO INIT"
9. Execute: `PAGE P`
10. Verify: Page switches to Pattern, output "SWITCHED TO PATTERN"
11. Execute: `PAGE LIVE`
12. Verify: Page switches to Live, output "SWITCHED TO LIVE"

**Expected:** All page names switch correctly ✅ / ❌

---

### Test 2.2: Page Aliases

**Test Steps:**
1. From current page, execute: `PAGE L`
2. Verify: Switches to Live page
3. Execute: `PAGE H`
4. Verify: Switches to Help page
5. Execute: `PG V`
6. Verify: Switches to Variables page
7. Execute: `PG N`
8. Verify: Switches to Notes page
9. Execute: `PG S`
10. Verify: Switches to Scope page

**Expected:** Short aliases work (L, H, PG) ✅ / ❌

---

### Test 2.3: GRID Command

**Test Steps:**
1. Navigate to Live page (if not already there)
2. Verify: Normal Live view (no grid)
3. Execute: `PAGE GRID` or `PAGE G`
4. Verify: Page shows Live page with grid view enabled
5. Toggle grid off: `GRID.VIEW 0`
6. Verify: Grid view disabled

**Expected:** PAGE GRID enables grid view on Live page ✅ / ❌

---

### Test 2.4: Script-Controlled Navigation

**Setup:**
1. Script 1 contains:
   ```
   PAGE 2
   PRINT NOW ON PAGE 2
   ```
2. Script 2 contains:
   ```
   PAGE LIVE
   PRINT BACK TO LIVE
   ```

**Test Steps:**
1. From Live page, execute: `SCRIPT 1`
2. Verify: Page switches to Script 2
3. Verify output: "NOW ON PAGE 2"
4. Execute: `SCRIPT 2`
5. Verify: Page switches to Live
6. Verify output: "BACK TO LIVE"

**Expected:** Scripts can control page navigation ✅ / ❌

---

### Test 2.5: Metro Page Switching

**Setup:**
1. Metro script contains:
   ```
   A ADD A 1
   IF EQ A 1: PAGE 1
   IF EQ A 2: PAGE 2
   IF EQ A 3: PAGE 3
   IF GT A 3: A 0
   ```
2. Variable A = 0

**Test Steps:**
1. Start at Live page
2. Start metro: `M.START 120`
3. Wait for first tick
4. Verify: Page switches to Script 1
5. Wait for second tick
6. Verify: Page switches to Script 2
7. Wait for third tick
8. Verify: Page switches to Script 3
9. Wait for fourth tick
10. Verify: Page switches back to Script 1 (cycle repeats)
11. Stop metro: `M.STOP`

**Expected:** Metro can drive page navigation ✅ / ❌

---

### Test 2.6: Invalid Page Names

**Test Steps:**
1. Execute: `PAGE 9` (invalid - no script 9)
2. Verify: Error message "UNKNOWN PAGE: 9"
3. Execute: `PAGE INVALID`
4. Verify: Error message "UNKNOWN PAGE: INVALID"
5. Execute: `PAGE X`
6. Verify: Error message "UNKNOWN PAGE: X"

**Expected:** Invalid page names produce errors ✅ / ❌

---

### Test 2.7: All Valid Page Names

**Test Steps:**
Execute each command and verify correct page:
1. `PAGE LIVE` → Live
2. `PAGE 1` → Script 1
3. `PAGE 2` → Script 2
4. `PAGE 3` → Script 3
5. `PAGE 4` → Script 4
6. `PAGE 5` → Script 5
7. `PAGE 6` → Script 6
8. `PAGE 7` → Script 7
9. `PAGE 8` → Script 8
10. `PAGE M` → Metro
11. `PAGE I` → Init
12. `PAGE P` → Pattern
13. `PAGE V` → Variables
14. `PAGE N` → Notes
15. `PAGE S` → Scope
16. `PAGE HELP` → Help

**Expected:** All page names work ✅ / ❌

---

## Feature 3: Compressor Dry/Wet Mix

### Test 3.1: Full Dry (Bypass)

**Setup:**
1. Create a loud test tone:
   - `PF 440` (A4)
   - `VOL 12000`
   - `TR`

**Test Steps:**
1. Set compressor aggressive: `CR 10` (10:1 ratio), `CT 2000` (low threshold)
2. Execute: `CR.MIX 0` (100% dry)
3. Listen: Audio should NOT be compressed (full dynamic range)
4. Execute: `CR.MIX 16383` (100% wet)
5. Listen: Audio should be heavily compressed (reduced dynamics)

**Expected:** CR.MIX 0 bypasses compression completely ✅ / ❌

---

### Test 3.2: Full Wet (Compressed)

**Setup:**
1. Same test tone as above
2. Compressor set aggressive

**Test Steps:**
1. Execute: `CR.MIX 16383`
2. Verify: Audio heavily compressed
3. This should sound identical to pre-v0.4.2 behavior (default)

**Expected:** CR.MIX 16383 = full compression (backward compatible) ✅ / ❌

---

### Test 3.3: Parallel Compression (50/50)

**Setup:**
1. Same test tone
2. Compressor aggressive: `CR 10`, `CT 2000`

**Test Steps:**
1. Execute: `CR.MIX 8192` (50% dry, 50% wet)
2. Listen: Should hear blend of dynamics and compression
3. Compare to full wet (CR.MIX 16383)
4. Verify: Less compressed than full wet, but more controlled than dry

**Expected:** Parallel compression creates natural blend ✅ / ❌

---

### Test 3.4: Sweep Dry to Wet

**Setup:**
1. Test tone playing continuously
2. Compressor aggressive

**Test Steps:**
1. Start at: `CR.MIX 0`
2. Slowly increase: `CR.MIX 4000`, `CR.MIX 8000`, `CR.MIX 12000`, `CR.MIX 16383`
3. Listen: Should hear smooth crossfade from uncompressed to compressed

**Expected:** Smooth transition, no clicks or pops ✅ / ❌

---

### Test 3.5: Expression Evaluation

**Test Steps:**
1. Execute: `CR.MIX ADD 8000 8000` (should = 16000)
2. Verify: Command succeeds
3. Execute: `CR.MIX MUL 2 4000` (should = 8000)
4. Verify: Command succeeds

**Expected:** Expressions work in CR.MIX ✅ / ❌

---

### Test 3.6: RND.FX Includes CR.MIX

**Test Steps:**
1. Execute: `RND.FX`
2. Observe: CR.MIX should be randomized (check with query or listen)
3. Execute `RND.FX` several more times
4. Verify: CR.MIX changes each time

**Expected:** RND.FX randomizes CR.MIX (favors wet 8192-16383) ✅ / ❌

---

### Test 3.7: RST Resets CR.MIX

**Test Steps:**
1. Set: `CR.MIX 0` (dry)
2. Execute: `RST`
3. Check default: Should reset to 16383 (wet)
4. Trigger tone: `TR`
5. Listen: Should be compressed (not dry)

**Expected:** RST sets CR.MIX to 16383 (default wet) ✅ / ❌

---

### Test 3.8: Scene Persistence

**Setup:**
1. Set unique CR.MIX value: `CR.MIX 6000`

**Test Steps:**
1. Save scene: `SAVE test_cr_mix`
2. Change CR.MIX: `CR.MIX 16383`
3. Load scene: `LOAD test_cr_mix`
4. Verify: CR.MIX restored to 6000

**Expected:** CR.MIX persists in scenes ✅ / ❌

---

### Test 3.9: Help System

**Test Steps:**
1. Navigate to Help page: `PAGE HELP`
2. Search for compressor section
3. Verify CR.MIX / CRMIX is documented
4. Verify description includes: "DRY/WET (0-16383)"
5. Verify note: "0=DRY 16383=WET (PARALLEL)"

**Expected:** Help system documents CR.MIX ✅ / ❌

---

## Feature 4: Title Timer Persistence

### Test 4.1: Enable Timer (No Restart)

**Test Steps:**
1. Execute: `TITLE.TIMER 1 5` (enable, 5 second interval)
2. Wait 5 seconds
3. Verify: Title toggles between "monokit" and BPM/CPU display
4. Wait another 5 seconds
5. Verify: Title toggles again

**Expected:** Timer works immediately after enabling ✅ / ❌

---

### Test 4.2: Timer Persists on Restart

**Test Steps:**
1. Enable timer: `TITLE.TIMER 1 3` (3 second interval)
2. Verify timer is working (title toggles)
3. Quit monokit
4. Restart monokit
5. Immediately after startup, wait 3 seconds
6. Verify: Title toggles (timer auto-started)

**Expected:** No "kickstarting" needed - timer starts immediately ✅ / ❌

---

### Test 4.3: Disabled Timer Stays Disabled

**Test Steps:**
1. Disable timer: `TITLE.TIMER 0`
2. Verify: Title shows static "monokit"
3. Quit monokit
4. Restart monokit
5. Verify: Title remains static (timer still disabled)

**Expected:** Disabled state persists correctly ✅ / ❌

---

### Test 4.4: Interval Changes Persist

**Test Steps:**
1. Set interval: `TITLE.TIMER 1 10` (10 seconds)
2. Wait to verify 10-second toggle
3. Quit monokit
4. Restart monokit
5. Time the toggle interval
6. Verify: Still 10 seconds (not default)

**Expected:** Custom interval persists ✅ / ❌

---

### Test 4.5: Toggle On/Off Multiple Times

**Test Steps:**
1. Enable: `TITLE.TIMER 1 2`
2. Wait 2 seconds, verify toggle
3. Disable: `TITLE.TIMER 0`
4. Verify: Title static
5. Enable: `TITLE.TIMER 1 2`
6. Wait 2 seconds, verify toggle again
7. Quit and restart
8. Verify: Timer still enabled and working

**Expected:** Toggle works reliably, last state persists ✅ / ❌

---

## Integration Tests

### Integration 1: Mute + Page Navigation

**Scenario:** Navigate to muted script page

**Test Steps:**
1. Mute script 3: `MUTE.3`
2. Navigate via command: `PAGE 3`
3. Verify: Title shows " SCRIPT 3 [MUTED] "
4. Unmute via hotkey: Alt+Shift+3
5. Verify: Title updates to " SCRIPT 3 "

**Expected:** Page navigation + mute state display work together ✅ / ❌

---

### Integration 2: Mute + Metro + Page Nav

**Scenario:** Metro switches pages, some scripts muted

**Setup:**
1. Script 1: `PRINT SCRIPT 1 EXECUTED`
2. Script 2: `PRINT SCRIPT 2 EXECUTED`
3. Metro: `PAGE 1; SCRIPT 1; PAGE 2; SCRIPT 2; PAGE LIVE`
4. Mute script 2: `MUTE.2`

**Test Steps:**
1. Start metro: `M.START 120`
2. Wait for tick
3. Verify: Switches to Script 1, "SCRIPT 1 EXECUTED"
4. Verify: Switches to Script 2, "SCRIPT 2 IS MUTED" (no execution)
5. Verify: Returns to Live page
6. Stop metro: `M.STOP`

**Expected:** Metro navigation works with muted scripts ✅ / ❌

---

### Integration 3: All Features in One Scene

**Scenario:** Save scene with all v0.4.2 features configured

**Setup:**
1. Mute scripts 2, 4, 6, M
2. Set CR.MIX to 10000 (parallel compression)
3. Enable timer: `TITLE.TIMER 1 5`
4. Navigate to Script 3: `PAGE 3`

**Test Steps:**
1. Save scene: `SAVE test_v0_4_2_all`
2. Change everything:
   - Unmute all scripts
   - Set CR.MIX 0
   - Disable timer: `TITLE.TIMER 0`
   - Navigate to Live: `PAGE LIVE`
3. Load scene: `LOAD test_v0_4_2_all`
4. Verify: Scripts 2, 4, 6, M are muted (query with `MUTE`)
5. Verify: Currently on Script 3 page
6. Verify: CR.MIX = 10000 (test with tone)
7. Wait 5 seconds
8. Verify: Title toggles (timer enabled)

**Expected:** All v0.4.2 features persist in scene ✅ / ❌

---

### Integration 4: Hotkeys Don't Conflict

**Scenario:** Ensure new hotkeys don't break existing ones

**Test Steps:**
1. Test existing hotkeys still work:
   - Alt+1 through Alt+8 → Script pages
   - Alt+M → Metro page
   - Alt+I → Init page
   - Ctrl+Z → Undo
   - Ctrl+Y → Redo
   - Ctrl+S → Save line
2. Test new hotkeys work:
   - Alt+Shift+1 through Alt+Shift+8 → Toggle mutes
   - Alt+Shift+M → Toggle metro mute
   - Alt+Shift+I → Toggle init mute

**Expected:** No hotkey conflicts ✅ / ❌

---

## Edge Cases & Error Handling

### Edge 1: Mute Non-Existent Script

**Test Steps:**
1. Execute: `MUTE 9` (no script 9)
2. Verify: Error "MUTE: INVALID SCRIPT ID"

**Expected:** Proper error handling ✅ / ❌

---

### Edge 2: CR.MIX Out of Range

**Test Steps:**
1. Execute: `CR.MIX -1000`
2. Verify: Value clamps to 0 (no error, just clips)
3. Execute: `CR.MIX 20000`
4. Verify: Value clamps to 16383

**Expected:** Values clamp to valid range ✅ / ❌

---

### Edge 3: PAGE During Script Execution

**Scenario:** Script switches pages while executing multi-line script

**Setup:**
1. Script 1 (5 lines):
   ```
   PRINT LINE 1
   PAGE 2
   PRINT LINE 3
   PAGE LIVE
   PRINT LINE 5
   ```

**Test Steps:**
1. Execute: `SCRIPT 1`
2. Observe page changes and output
3. Verify: All 5 lines execute, page changes happen

**Expected:** Page switching during execution works ✅ / ❌

---

### Edge 4: Rapid Mute Toggling

**Test Steps:**
1. Rapidly press Alt+Shift+1 ten times
2. Verify: Final state is consistent
3. Execute: `MUTE` (query)
4. Verify: Script 1 shows correct state (muted or active based on odd/even toggles)

**Expected:** Rapid toggling doesn't desync state ✅ / ❌

---

## Regression Tests

### Regression 1: Existing Commands Still Work

**Test Steps:**
Verify these existing commands work:
1. `TR` - Trigger voice
2. `PF 440` - Set frequency
3. `RST` - Reset parameters
4. `SAVE test` - Save scene
5. `LOAD test` - Load scene
6. `BPM 140` - Set tempo
7. `M.START` - Start metro
8. `RND.FX` - Randomize effects

**Expected:** No regressions in existing functionality ✅ / ❌

---

### Regression 2: Scene Backward Compatibility

**Setup:**
1. Create old scene (pre-v0.4.2) without new fields
2. Manually create JSON without script_mutes, cr_mix, etc.

**Test Steps:**
1. Load old scene
2. Verify: Loads without errors
3. Verify: New features use defaults (unmuted, CR.MIX 16383)

**Expected:** Old scenes load correctly with sensible defaults ✅ / ❌

---

## Performance Tests

### Performance 1: Mute Overhead

**Test Steps:**
1. Create 8 scripts with heavy operations (loops, math)
2. Mute all 8 scripts
3. Metro triggers all 8: `SCRIPT 1; SCRIPT 2; ... SCRIPT 8`
4. Observe: Should be very fast (instant "MUTED" messages)
5. Unmute all
6. Metro triggers all 8
7. Compare: Muted execution should be noticeably faster

**Expected:** Muted scripts have minimal overhead ✅ / ❌

---

### Performance 2: Page Switching Speed

**Test Steps:**
1. Rapidly execute: `PAGE 1; PAGE 2; PAGE 3; PAGE LIVE`
2. Observe: Should be instant, no lag

**Expected:** Page switching is fast ✅ / ❌

---

## Final Verification

### Build & Test Summary
- [ ] All 602 automated tests pass
- [ ] Build succeeds (debug + release)
- [ ] No compiler warnings (except expected)
- [ ] Help system complete for all new commands
- [ ] Documentation updated (CHANGELOG, ROADMAP)

### Manual Test Summary
- [ ] Script Mutes: ___/9 tests passed
- [ ] Page Navigation: ___/7 tests passed
- [ ] Compressor Dry/Wet: ___/9 tests passed
- [ ] Title Timer: ___/5 tests passed
- [ ] Integration: ___/4 tests passed
- [ ] Edge Cases: ___/4 tests passed
- [ ] Regression: ___/2 tests passed
- [ ] Performance: ___/2 tests passed

**Total:** ___/42 manual tests passed

---

## Issues Found

| # | Test | Issue Description | Severity | Status |
|---|------|-------------------|----------|--------|
| 1 |      |                   |          |        |
| 2 |      |                   |          |        |
| 3 |      |                   |          |        |

---

## Sign-Off

**Tester:** _______________
**Date:** _______________
**Result:** PASS / FAIL / CONDITIONAL PASS
**Notes:**

---

## Quick Smoke Test (5 minutes)

If time is limited, run this abbreviated test:

1. **Mutes:** `MUTE.1`, Alt+1, verify "[MUTED]", Alt+Shift+1, verify title changes
2. **Page Nav:** `PAGE 2`, verify switch, `PAGE LIVE`, verify switch
3. **Compressor:** `TR`, `CR.MIX 0`, listen (dry), `CR.MIX 16383`, listen (wet)
4. **Timer:** `TITLE.TIMER 1 3`, wait 3 sec, verify toggle
5. **Integration:** Save scene with mutes, restart, load scene, verify persistence

**Quick smoke test result:** PASS / FAIL
