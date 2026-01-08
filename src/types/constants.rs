#[cfg(not(feature = "scsynth-direct"))]
pub const OSC_ADDR: &str = "127.0.0.1:57120";

#[cfg(feature = "scsynth-direct")]
pub const OSC_ADDR: &str = "127.0.0.1:57110";

pub const MONOKIT_NODE_ID: i32 = 1000;
pub const SPECTRUM_BANDS: usize = 15;
pub const SCOPE_SAMPLES: usize = 128;

pub const TIER_SILENT: u8 = 0;
pub const TIER_ERRORS: u8 = 1;
pub const TIER_ESSENTIAL: u8 = 2;
pub const TIER_QUERIES: u8 = 3;
pub const TIER_CONFIRMS: u8 = 4;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OutputCategory {
    Error,
    Essential,
    Query,
    Confirm,
    Verbose,
}

pub const GRID_ICONS: [char; 48] = [
    '~', '≈', '∿', '∞', '×', '⟲', '↯', '⊂',
    '◎', '⊏', '⊐', '∥', '⊥', '⊡', '▼', '↘',
    '↑', '↓', '↗', '◢', '◣', '⋮', '⟳', '◐',
    '⌓', '⌐', '◑', '⊞', '▣', '⇆', '⤴', '⊛',
    '⊟', '⊠', '≡', '⊕', '⫰', '⧫', '⧪', '⬡',
    '⬢', '⬣', '▮', '⬌', '⟿', '✱', '◉', '⊙',
];

pub const GRID_LABELS: [&str; 48] = [
    "PF", "PW", "MF", "MW", "FM", "FB", "DC", "FC",
    "FQ", "FT", "FE", "RF", "RD", "RM", "AD", "PD",
    "PA", "FD", "FA", "DD", "DA", "DT", "DF", "DW",
    "RV", "RH", "RW", "LM", "CT", "BM", "PS", "RG",
    "LB", "LS", "EQ", "TK", "MB", "MP", "MD", "MT",
    "MA", "MX", "VL", "PN", "DS", "MM", "ME", "FK",
];
