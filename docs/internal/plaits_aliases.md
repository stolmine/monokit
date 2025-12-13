# Plaits Parameter Aliases (Planned v0.3.7)

## 3-Letter Aliases

| Alias | Full Command | Parameter | Range |
|-------|--------------|-----------|-------|
| **PLE** | PL.ENG | Engine selection | 0-15 |
| **PLH** | PL.HARM | Harmonics | 0-16383 |
| **PLT** | PL.TIMB | Timbre | 0-16383 |
| **PLM** | PL.MORPH | Morph | 0-16383 |
| **PLD** | PL.DEC | Decay envelope | 0-16383 |
| **PLL** | PL.LPG | Lowpass gate | 0-16383 |

## Existing Short Forms

| Command | Parameter | Notes |
|---------|-----------|-------|
| **PLV** | Plaits main volume | Already short |
| **PAV** | Plaits aux volume | Already short |
| **PLTR** | Plaits trigger | Already short |

## Pitch Control (New)

| Command | Parameter | Range | Notes |
|---------|-----------|-------|-------|
| **PL.FREQ** | Pitch/frequency | +/- 14-bit, N | Scale-aware, independent from PF |

## Design Rationale

- **3-letter format** distinguishes Plaits params from Complex oscillator (2-letter)
- **PL prefix** maintains namespace consistency with existing PL.* commands
- **Single letter suffix** indicates parameter type (H=harmonics, T=timbre, etc.)
- **Improves legibility** in dense script files
- **Backwards compatible** - full commands (PL.HARM, etc.) still work

## Trigger Indicators (Planned)

- **P** = Plaits trigger active
- **C** = Complex oscillators trigger active (replaces "TR")
- Header shows: `[P ]` or `[ C]` or `[PC]` for layered triggers

## Terminology

- **Complex** = Dual FM oscillator architecture (formerly "HD2")
- **Plaits** = Mutable Instruments macro oscillator
