# Command Vocabulary Consistency Audit

Comprehensive audit of naming conventions, parameter ranges, and alias patterns.

---

## Executive Summary

**Major Issues Found:**
1. Parameter ranges inconsistent across voices (Hz vs 14-bit vs ms)
2. Envelope timing uses different units (HD2=ms, Sampler=14-bit)
3. Alias patterns unpredictable (2-4 letters with no clear rule)
4. Several orphaned/bidirectional canonical forms
5. Modulation routing commands too similar (MP/MD/MT/MA/MC/MQ)

---

## 1. Parameter Range Inconsistencies

### CRITICAL: Volume Parameters

| Parameter | Range | Type | Issue |
|-----------|-------|------|-------|
| VOL (master) | 0.0-1.0 | float | **Different from all others** |
| VOL.OSC | 0-16383 | 14-bit | |
| PLV | 0-16383 | 14-bit | |
| PAV | 0-16383 | 14-bit | |
| NV | 0-16383 | 14-bit | |
| VOL.SMP | 0-16383 | 14-bit | |

**Recommendation:** Change VOL to 0-16383 for consistency, or document clearly.

### CRITICAL: Filter Cutoff

| Parameter | Range | Type | Issue |
|-----------|-------|------|-------|
| FC (HD2) | 20-20000 | Hz | **Raw frequency** |
| SF.CUT (Sampler) | 0-16383 | 14-bit | **Mapped internally** |

**Recommendation:** Standardize both to Hz or both to 14-bit.

### CRITICAL: Envelope Timing

| Voice | Attack | Decay | Release | Unit |
|-------|--------|-------|---------|------|
| HD2 | 1-10000 | 1-10000 | - | **milliseconds** |
| Sampler | 0-16383 | 0-16383 | 0-16383 | **14-bit (scaled)** |

**Recommendation:** Standardize all envelopes to milliseconds for intuitive use.

### MINOR: Pan Ranges

| Parameter | Range | Issue |
|-----------|-------|-------|
| PAN (master) | -16383 to +16383 | Full symmetric |
| PAN.OSC/PLA/NOS/SMP | -8192 to +8191 | **Asymmetric, half range** |

**Recommendation:** Use consistent range across all pan parameters.

### MINOR: Pitch Parameters

| Parameter | Range | Unit | Notes |
|-----------|-------|------|-------|
| PF | 20-20000 | Hz | Raw frequency |
| PL.FREQ | 20-20000 | Hz | Consistent with PF ✓ |
| S.PITCH | -24 to +24 | semitones | **Different concept** |
| S.FINE | -100 to +100 | cents | Fine tuning |

**Note:** S.PITCH using semitones makes sense for sample transposition. Consider if PF/PL.FREQ should also accept note numbers.

---

## 2. Naming Convention Issues

### Orphaned Commands (No Canonical Form)

| Command | Should Be | Notes |
|---------|-----------|-------|
| PLV | PL.VOL | Plaits volume has no long form |
| PAV | PL.AUX | Plaits aux volume has no long form |
| EQ | EQ.Q or EQ.MQ | Mid Q control, breaks pattern |

### Bidirectional Canonical (Confusing)

| Form A | Form B | Issue |
|--------|--------|-------|
| COMP.AUTO | CAU | Both point to each other |
| SCOPE.GAIN | SCG | Both registered separately |
| SCOPE.RST | SCR | Both registered separately |

**Recommendation:** Pick one direction. Short → Long is standard.

### Duplicate Aliases

| Alias 1 | Alias 2 | Canonical | Issue |
|---------|---------|-----------|-------|
| CR.MIX | CRMIX | COMP.MIX | Two forms for same command |

### Inconsistent Prefix Usage

| Category | Pattern | Exceptions |
|----------|---------|------------|
| Compressor | CR.* or COMP.* | CR.MIX vs COMP.AUTO - mixed |
| Clouds | CL.* | Consistent ✓ |
| Plaits | PL.* | PLV/PAV break pattern |
| Sampler | S.* | SF.*, SRINGS.* are subgroups |

---

## 3. Alias Length Inconsistencies

### Envelope Attack (Inconsistent)

| Decay (2 letter) | Attack (3-4 letter) | Why Different? |
|------------------|---------------------|----------------|
| AD | AA | OK |
| PD | PAA | 3 letters |
| FD | FAA | 3 letters |
| DD | DAA | 3 letters |
| FBD | FBAA | 4 letters |
| FED | FLAA | 4 letters |

**Recommendation:** Standardize to 2-letter where possible, or document the conflict.

### Sampler Commands (Wildly Inconsistent)

| Length | Examples | Count |
|--------|----------|-------|
| 2 | SR, SD, SL, SA | ~6 |
| 3 | SPT, SFN, SST, SLE, SDC, SRE | ~10 |
| 4 | SSLC, SONS, SRRP, SRRS | ~10 |
| 5 | SOMIN, SFCM, SFQM | ~3 |

**Recommendation:** Establish clear rule (e.g., all sampler = 2-3 letters max).

### Commands Missing Short Aliases

| Command | Suggested Alias |
|---------|-----------------|
| SRINGS.PIT | SRP (currently SRRP - 4 letters) |
| SRINGS.STRC | SRS |
| SRINGS.BRIT | SRB |
| SRINGS.DAMP | SRD |
| SRINGS.POS | SRO |
| SRINGS.MODE | SRM |
| SRINGS.WET | SRW |
| FMEV.CRV | FC (conflicts with filter) |
| DENV.CRV | DC (conflicts with disc) |
| FBEV.AMT | FBA (conflicts with feedback) |

---

## 4. Modulation Routing Confusion

All these commands start with M and are hard to distinguish:

| Alias | Meaning | Mnemonic |
|-------|---------|----------|
| MP | Route to Pitch | M=mod, P=pitch |
| MD | Route to Discontinuity | M=mod, D=disc |
| MT | Route to Timbre | M=mod, T=timbre |
| MA | Route to Amplitude | M=mod, A=amp |
| MC | Route to Filter Cutoff | M=mod, C=cut |
| MQ | Route to Filter Q | M=mod, Q=resonance |
| MB | Mod Bus Amount | M=mod, B=bus |
| MBA | Mod Bus Env Amount | Collision with MB |
| MBD | Mod Bus Env Decay | |
| MX | Mix Routing | |
| MM | Mod Mix Routing | |
| ME | Effect Mix Routing | |
| MF | Mod Oscillator Freq | |
| MW | Mod Oscillator Wave | |
| MV | Mod Volume | |
| MFF | Mod Filter Cutoff Amount | |
| MFQ | Mod Filter Q Amount | |

**Problem:** 17 commands starting with M. Users can't predict which M* does what.

**Recommendation:** Consider restructuring:
- Routing: R.PIT, R.DIS, R.TIM, R.AMP, R.CUT, R.RES
- Mod Bus: B.AMT, B.ATK, B.DEC
- Mod Osc: MO.F, MO.W, MO.V (or keep MF, MW, MV)

---

## 5. Proposed Standardization Rules

### Rule 1: Parameter Ranges

| Category | Standard Range | Unit |
|----------|----------------|------|
| Frequencies | 20-20000 | Hz |
| Time/Envelope | 1-10000 | ms |
| Amounts/Levels | 0-16383 | 14-bit |
| Pan | -8192 to +8191 | 14-bit centered |
| Pitch offset | -24 to +24 | semitones |
| Fine pitch | -100 to +100 | cents |
| Boolean | 0-1 | |
| Mode/Type | 0-N | enumerated |

### Rule 2: Alias Length

| Category | Target Length | Pattern |
|----------|---------------|---------|
| Core synth params | 2 letters | PF, MW, FC |
| Envelope params | 2-3 letters | AD, PA, FED |
| Effect params | 2-3 letters | DT, RV, BRL |
| Sampler params | 2-3 letters | SR, SPT |
| System commands | 3-4 letters | CLR, SYNC |

### Rule 3: Prefix Conventions

| Prefix | Module | Example |
|--------|--------|---------|
| P* | Primary oscillator | PF, PW, PV |
| M* | Modulation oscillator | MF, MW, MV |
| N* | Noise | NW, NV |
| PL* | Plaits | PLF, PLV |
| S* | Sampler | SR, SPT |
| SF* | Sampler filter | SFC, SFQ |
| SR* | Sampler Rings | SRP, SRW |
| F* | Filter | FC, FQ, FT |
| D* | Delay | DT, DF, DW |
| R* | Reverb | RV, RW, RP |
| C* | Compressor | CT, CR, CM |
| E* | EQ | EL, EM, EH |
| B* | Beat repeat | BRL, BRX |
| CL* | Clouds | CLP, CLW |

### Rule 4: Envelope Naming

| Envelope | Amount | Decay | Attack | Curve |
|----------|--------|-------|--------|-------|
| Amplitude | - | AD | AA | AC |
| Pitch | PA | PD | PAT | PC |
| FM | FA | FD | FAT | FC* |
| Discontinuity | DA | DD | DAT | DC* |
| Filter | FE | FED | FEA | FEC |
| Feedback | FBA | FBD | FBAT | FBC |
| ModBus | MBA | MBD | - | - |

*Note: FC and DC conflict with Filter Cutoff and Discontinuity amount.

---

## 6. Implementation Priority

### Phase 1: Critical (Breaking Changes)
1. Standardize VOL to 14-bit (0-16383)
2. Standardize sampler envelope timing to ms
3. Standardize filter cutoff format

### Phase 2: Important (Cleanup)
1. Add canonical forms for PLV → PL.VOL, PAV → PL.AUX
2. Remove duplicate CR.MIX/CRMIX (keep one)
3. Fix bidirectional COMP.AUTO/CAU
4. Shorten SRINGS aliases (SRRP → SRP)

### Phase 3: Polish (Consistency)
1. Standardize attack envelope aliases
2. Restructure modulation routing commands
3. Add missing envelope curve aliases
4. Document all ranges in help system

### Phase 4: Documentation
1. Update MANUAL.md with standardized ranges
2. Add "Command Naming Conventions" section
3. Create quick reference card

---

## 7. Migration Considerations

**Breaking changes require:**
1. Version bump (0.6.0)
2. Scene migration script (update saved parameter values)
3. Deprecation warnings for old forms
4. COMPAT.MODE flag for legacy scenes

**Non-breaking changes:**
- Adding aliases (always safe)
- Adding canonical forms (safe)
- Updating help text (safe)

---

## Files Referenced

- `src/commands/registry/*.rs` - Command definitions
- `src/commands/synth/*.rs` - Parameter handlers
- `build_scripts/synthdefs/*.scd` - SuperCollider ranges
- `src/commands/aliases.rs` - Alias resolution
