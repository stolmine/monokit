//! System, I/O, and file management commands

use super::{ArgCount, CommandDef};

pub fn register_system(m: &mut std::collections::HashMap<&'static str, CommandDef>) {
    // Metro
    m.insert("M", CommandDef::new("M", None, ArgCount::Range(0, 1), "Metro toggle/query"));
    m.insert("M.BPM", CommandDef::new("M.BPM", None, ArgCount::Range(0, 1), "Metro BPM"));
    m.insert("M.ACT", CommandDef::new("M.ACT", None, ArgCount::Range(0, 1), "Metro active script"));
    m.insert("M.SCRIPT", CommandDef::new("M.SCRIPT", None, ArgCount::Range(0, 1), "Metro script"));
    m.insert("M.SYNC", CommandDef::new("M.SYNC", None, ArgCount::Custom, "Metro sync (unvalidated)"));

    // MIDI
    m.insert("MIDI", CommandDef::new("MIDI", None, ArgCount::Custom, "MIDI input config"));
    m.insert("MIDI.IN", CommandDef::new("MIDI.IN", None, ArgCount::Custom, "MIDI input config"));
    m.insert("MIDI.DIAG", CommandDef::new("MIDI.DIAG", None, ArgCount::Range(1, 2), "MIDI diagnostics"));

    // Audio
    m.insert("AUDIO", CommandDef::new("AUDIO", None, ArgCount::Custom, "Audio output config"));
    m.insert("AUDIO.OUT", CommandDef::new("AUDIO.OUT", None, ArgCount::Custom, "Audio output config"));

    // SuperCollider
    m.insert("SC.DIAG", CommandDef::new("SC.DIAG", None, ArgCount::Range(1, 2), "SC diagnostics"));

    // Scene Management
    m.insert("SAVE", CommandDef::new("SAVE", None, ArgCount::Exactly(1), "Save scene"));
    m.insert("LOAD", CommandDef::new("LOAD", None, ArgCount::Exactly(1), "Load scene"));
    m.insert("DELETE", CommandDef::new("DELETE", None, ArgCount::Exactly(1), "Delete scene"));
    m.insert("SCENES", CommandDef::new("SCENES", None, ArgCount::None, "List scenes"));
    m.insert("LOAD.RST", CommandDef::new("LOAD.RST", None, ArgCount::Range(0, 1), "RST on load"));
    m.insert("LOAD.CLR", CommandDef::new("LOAD.CLR", None, ArgCount::Range(0, 1), "Clear on load"));
    m.insert("AUTOLOAD", CommandDef::new("AUTOLOAD", None, ArgCount::Range(0, 1), "Autoload scene"));

    // Preset Management
    m.insert("PSET", CommandDef::new("PSET", None, ArgCount::AtLeast(2), "Recall preset"));
    m.insert("PSET.SAVE", CommandDef::new("PSET.SAVE", None, ArgCount::AtLeast(2), "Save preset"));
    m.insert("PSET.DEL", CommandDef::new("PSET.DEL", None, ArgCount::AtLeast(1), "Delete preset"));
    m.insert("PSETS", CommandDef::new("PSETS", None, ArgCount::None, "List presets"));

    // Recording
    m.insert("REC", CommandDef::new("REC", None, ArgCount::None, "Start recording"));
    m.insert("REC.STOP", CommandDef::new("REC.STOP", None, ArgCount::None, "Stop recording"));
    m.insert("REC.PATH", CommandDef::new("REC.PATH", None, ArgCount::Exactly(1), "Set recording path"));

    // Quantization
    m.insert("Q.ROOT", CommandDef::new("Q.ROOT", None, ArgCount::Range(0, 1), "Quantize root note"));
    m.insert("Q.SCALE", CommandDef::new("Q.SCALE", None, ArgCount::Exactly(1), "Quantize scale"));
    m.insert("Q.BIT", CommandDef::new("Q.BIT", None, ArgCount::Exactly(1), "Set scale bit"));

    // Delayed Execution
    m.insert("DEL", CommandDef::new("DEL", None, ArgCount::Custom, "Delay command"));
    m.insert("DEL.X", CommandDef::new("DEL.X", None, ArgCount::Custom, "Delay execute N times"));
    m.insert("DEL.R", CommandDef::new("DEL.R", None, ArgCount::Custom, "Delay repeat every N"));
    m.insert("DEL.CLR", CommandDef::new("DEL.CLR", None, ArgCount::None, "Clear all delays"));

    // Sync
    m.insert("SYNC", CommandDef::new("SYNC", None, ArgCount::None, "Sync all patterns/counters"));
    m.insert("SYNC.SEQ", CommandDef::new("SYNC.SEQ", None, ArgCount::None, "Sync pattern sequences"));
    m.insert("SYNC.TOG", CommandDef::new("SYNC.TOG", None, ArgCount::None, "Sync pattern toggles"));
    m.insert("SYNC.PAT", CommandDef::new("SYNC.PAT", None, ArgCount::None, "Sync pattern indices"));

    // Muting
    m.insert("MUTE", CommandDef::new("MUTE", None, ArgCount::Range(0, 2), "Mute script"));
    m.insert("MUTE.1", CommandDef::new("MUTE.1", None, ArgCount::Range(0, 1), "Mute script 1"));
    m.insert("MUTE.2", CommandDef::new("MUTE.2", None, ArgCount::Range(0, 1), "Mute script 2"));
    m.insert("MUTE.3", CommandDef::new("MUTE.3", None, ArgCount::Range(0, 1), "Mute script 3"));
    m.insert("MUTE.4", CommandDef::new("MUTE.4", None, ArgCount::Range(0, 1), "Mute script 4"));
    m.insert("MUTE.5", CommandDef::new("MUTE.5", None, ArgCount::Range(0, 1), "Mute script 5"));
    m.insert("MUTE.6", CommandDef::new("MUTE.6", None, ArgCount::Range(0, 1), "Mute script 6"));
    m.insert("MUTE.7", CommandDef::new("MUTE.7", None, ArgCount::Range(0, 1), "Mute script 7"));
    m.insert("MUTE.8", CommandDef::new("MUTE.8", None, ArgCount::Range(0, 1), "Mute script 8"));
    m.insert("MUTE.M", CommandDef::new("MUTE.M", None, ArgCount::Range(0, 1), "Mute metro script"));
    m.insert("MUTE.I", CommandDef::new("MUTE.I", None, ArgCount::Range(0, 1), "Mute init script"));

    // Page navigation
    m.insert("PAGE", CommandDef::new("PAGE", None, ArgCount::Exactly(1), "Switch page"));
    m.insert("PG", CommandDef::new("PG", Some("PAGE"), ArgCount::Exactly(1), "Switch page"));

    // Compatibility
    m.insert("COMPAT", CommandDef::new("COMPAT", None, ArgCount::None, "Show compatibility"));
    m.insert("COMPAT.MODE", CommandDef::new("COMPAT.MODE", None, ArgCount::Range(0, 1), "Compatibility mode"));

    // Version
    m.insert("VERSION", CommandDef::new("VERSION", None, ArgCount::None, "Show version"));
    m.insert("VER", CommandDef::new("VER", None, ArgCount::None, "Show version"));

    // Help
    m.insert("HELP", CommandDef::new("HELP", None, ArgCount::None, "Show help"));

    // Output control
    m.insert("LIMIT", CommandDef::new("LIMIT", None, ArgCount::Range(0, 1), "Output limiter"));

    // Confirmation dialogs
    m.insert("CFM.QUIT", CommandDef::new("CFM.QUIT", None, ArgCount::Range(0, 1), "Confirm quit w/ unsaved"));
    m.insert("CFM.SAVE", CommandDef::new("CFM.SAVE", None, ArgCount::Range(0, 1), "Confirm scene overwrite"));
}
