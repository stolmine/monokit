# Monokit Voice Architecture (v0.4.0)

## Multi-Synth Architecture

Monokit uses a 5-synth architecture with audio bus routing for independent voice control:

```
┌─────────────────────────────────────────────────────────────────┐
│                    RUST CLI (Command Layer)                     │
│  Commands: TR, PLTR, PF, PLF, PLH, NW, etc.                    │
└──────────────────────────┬──────────────────────────────────────┘
                           │ OSC Messages (/n_set)
                           │ Parameter → Node Routing
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│                SUPERCOLLIDER (5-Synth Engine)                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐      │
│  │monokit_noise  │  │ monokit_mod   │  │monokit_primary│      │
│  │  (Node 1002)  │  │  (Node 1003)  │  │  (Node 1004)  │      │
│  ├───────────────┤  ├───────────────┤  ├───────────────┤      │
│  │ Params:       │  │ Params:       │  │ Params:       │      │
│  │  NW, NV       │  │  MF, MW, MV   │  │  PF, PW, PV   │      │
│  │               │  │  FB, FBA, FBD │  │  FM, FA, FD   │      │
│  │ Noise Gen     │  │  MB, MBA, MBD │  │  PA, PD       │      │
│  │ W/P/B (NW)    │  │               │  │  DC, DM, DD   │      │
│  │      │        │  │ FM Osc        │  │  TK           │      │
│  │      ▼        │  │ Sin/Tri/Saw/FB│  │               │      │
│  │  Out→Bus 18   │  │      │        │  │ Complex Osc   │      │
│  └───────────────┘  │      ▼        │  │ Sin/Tri/Saw   │      │
│                     │  Out→Bus 17   │  │ + FM + Pitch  │      │
│                     └───────────────┘  │ Envelope      │      │
│                                        │      │        │      │
│  ┌───────────────┐                     │      ▼        │      │
│  │monokit_plaits │                     │  Out→Bus 16   │      │
│  │  (Node 1005)  │                     └───────────────┘      │
│  ├───────────────┤                                             │
│  │ Params:       │                                             │
│  │  PLF (pitch)  │  NEW in v0.4.0!                            │
│  │  PLE (engine) │  3-letter aliases:                         │
│  │  PLH (harm)   │  PLH, PLT, PLE                             │
│  │  PLT (timb)   │  PLM, PLD, PLL, PLF                        │
│  │  PLM (morph)  │                                             │
│  │  PLD (decay)  │                                             │
│  │  PLL (lpg)    │                                             │
│  │  PLV, PAV     │                                             │
│  │               │                                             │
│  │ MiPlaits      │                                             │
│  │ 16 Engines    │                                             │
│  │  Main │  AUX  │                                             │
│  │    ▼     ▼    │                                             │
│  │ Out→19 Out→20 │                                             │
│  └───────────────┘                                             │
│         │    │                                                 │
│  ┌──────┴────┴─────────────────────────────────────────┐      │
│  │              monokit_main (Node 1006)               │      │
│  ├─────────────────────────────────────────────────────┤      │
│  │ Inputs:                                             │      │
│  │   Bus 16 (Primary) + Bus 17 (Mod) +                │      │
│  │   Bus 18 (Noise) + Bus 19/20 (Plaits Main/AUX)     │      │
│  │                                                      │      │
│  │ ┌───────────────────────────────────────────────┐  │      │
│  │ │          SIGNAL PROCESSING CHAIN              │  │      │
│  │ ├───────────────────────────────────────────────┤  │      │
│  │ │                                               │  │      │
│  │ │  Source Mix (PV + MV + NV + Plaits)          │  │      │
│  │ │              ▼                                │  │      │
│  │ │  Discontinuity (DC + DA/DD, DM mode)         │  │      │
│  │ │              ▼                                │  │      │
│  │ │  Lo-Fi (LB/LS/LM)                            │  │      │
│  │ │              ▼                                │  │      │
│  │ │  Filter (FC/FQ/FT/FE/FK)                     │  │      │
│  │ │    14 types: SVF, Moog, RLPF, DFM1,          │  │      │
│  │ │              BMoog, Latch-SC                  │  │      │
│  │ │    ModBus routing: MF_F, MF_Q                │  │      │
│  │ │              ▼                                │  │      │
│  │ │  Ring Mod (RGF/RGW/RGM)                      │  │      │
│  │ │              ▼                                │  │      │
│  │ │  Comb Resonator (RF/RD/RM/RK)                │  │      │
│  │ │              ▼                                │  │      │
│  │ │  VCA (AD + VOLUME + MA modbus)               │  │      │
│  │ │   Mode: gate/drone                           │  │      │
│  │ │              ▼                                │  │      │
│  │ │  Compressor (CT/CR/CA/CL/CM)                 │  │      │
│  │ │              ▼                                │  │      │
│  │ │  Pan (PN)                                     │  │      │
│  │ │         ┌────┴────┐                          │  │      │
│  │ │         L         R                          │  │      │
│  │ │         ▼         ▼                          │  │      │
│  │ │  Beat Repeat (BR.LEN/REV/WIN/MIX)           │  │      │
│  │ │              ▼                                │  │      │
│  │ │  Pitch Shift (PS.MODE/SEMI/GRAIN/MIX/TARG)  │  │      │
│  │ │              ▼                                │  │      │
│  │ │  Stereo Delay (DT/DF/DLP/DW/DMODE/DTAIL)    │  │      │
│  │ │              ▼                                │  │      │
│  │ │  3-Band EQ (EL/EM/EF/EQ/EH)                  │  │      │
│  │ │              ▼                                │  │      │
│  │ │  Plate Reverb (RV/RP/RH/RW/RMODE/RTAIL)     │  │      │
│  │ │              ▼                                │  │      │
│  │ │  Limiter / DC Block (LIMIT)                  │  │      │
│  │ │              ▼                                │  │      │
│  │ │          Audio Out                           │  │      │
│  │ │                                               │  │      │
│  │ └───────────────────────────────────────────────┘  │      │
│  │                                                      │      │
│  │ 60+ Effect Parameters                               │      │
│  └─────────────────────────────────────────────────────┘      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

Trigger Commands:
  TR   → Triggers: monokit_noise + monokit_mod +
                   monokit_primary + monokit_main
  PLTR → Triggers: monokit_plaits only

UI Indicators (v0.4.0):
  C → Complex oscillators active (TR triggered)
  P → Plaits voice active (PLTR triggered)
```

See full parameter reference and routing table in docs/ARCHITECTURE.md

## Architecture Benefits

This multi-synth approach provides:

1. **Isolated Parameter Spaces** - No cross-talk between voice parameters
2. **Independent Triggering** - TR and PLTR can trigger different voice combinations  
3. **Parallel Development** - New voices can be added without affecting existing ones
4. **Efficient CPU Usage** - Only active voices consume processing power
5. **Clear Signal Flow** - Audio bus routing makes signal path explicit
6. **Flexible Routing** - Parameters automatically route to correct nodes
7. **Real-time Indicators** - C/P indicators show which voices are active

## Total Parameter Count: 97

- Complex Oscillators: ~30 parameters
- Plaits Voice: 9 parameters (+ 2 triggers)
- Main Effects Chain: ~60 parameters
